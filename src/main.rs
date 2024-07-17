use std::{cmp::Ordering, fmt::Display, fs, io::Write, str::FromStr};

use chrono::{DateTime, Months, Utc};
use clap::Parser;
use suppaftp::list::File;
use suppaftp::FtpStream;
mod numbering;
use numbering::get_series;
mod versioning;
use versioning::{parse_version, Version};

#[derive(Parser, Debug)]
struct Args {
    /// Spec numbering, e.g. 36.331
    spec: String,
    /// Release, e.g. 18
    #[arg(short, long, value_name = "REL")]
    rel: Option<u8>,
    /// Year and month, e.g. 2024-06
    /// When provided, it looks for specs
    /// from the given month to the given month plus two
    #[arg(short, long, value_name = "YYYY-MM", verbatim_doc_comment)]
    date: Option<String>,
    /// List all corresponding specs instead of downloading one
    #[arg(short, long)]
    list: bool,
}

#[derive(Debug)]
struct ParsedFile<'a> {
    name: &'a str,
    version: Version,
    date_time: DateTime<Utc>,
}

impl Display for ParsedFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, version: {}, date: {}",
            self.name,
            self.version,
            self.date_time.format("%Y-%m")
        )
    }
}

fn main() {
    let args = Args::parse();

    let start_date = match args.date {
        None => None,
        Some(str) => Some(
            DateTime::parse_from_str(
                format!("{str}-01 00:00:00 +0000").as_str(),
                "%Y-%m-%d %H:%M:%S %z",
            )
            .expect(format!("Date must be in a form of YYYY-MM, but got {str}").as_str()),
        ),
    };
    let end_date = match start_date {
        None => None,
        Some(date) => Some(date.clone().checked_add_months(Months::new(2)).unwrap()),
    };

    let series = get_series(&args.spec).unwrap();
    let path = format!("Specs/archive/{series}_series/{}", args.spec);

    let mut ftp_stream =
        FtpStream::connect("ftp.3gpp.org:21").expect("Failed to connect to ftp.3gpp.org");
    ftp_stream
        .login("anonymous", "anonymous")
        .expect("Failed to login to ftp.3gpp.org");

    let list = ftp_stream
        .list(Some(&path))
        .expect(format!("Failed to list files in '{path}'").as_str())
        .iter()
        .map(|entry| File::from_str(entry).unwrap())
        .filter(|file| file.name().ends_with(".zip") && file.is_file())
        .collect::<Vec<File>>();
    let mut parsed_list = list
        .iter()
        .map(|file| {
            let name = file.name();
            let last_index_of_hyphen = name.rfind("-").unwrap();
            let last_index_of_dot = name.rfind(".").unwrap();
            let version =
                parse_version(&name[last_index_of_hyphen + 1..last_index_of_dot]).unwrap();
            let date_time = DateTime::<Utc>::from(file.modified());
            ParsedFile {
                name,
                version,
                date_time,
            }
        })
        .filter(|file| {
            match args.rel {
                Some(rel) => {
                    if rel != file.version.major {
                        return false;
                    }
                }
                None => {}
            };
            match (start_date, end_date) {
                (Some(start), Some(end)) => {
                    if start > file.date_time || end < file.date_time {
                        return false;
                    }
                }
                _ => {}
            }
            true
        })
        .collect::<Vec<ParsedFile>>();
    parsed_list.sort_by(|a, b| {
        let major_ordering = b.version.major.cmp(&a.version.major);
        if major_ordering != Ordering::Equal {
            return major_ordering;
        }
        let minor_ordering = b.version.minor.cmp(&a.version.minor);
        if minor_ordering != Ordering::Equal {
            return minor_ordering;
        }
        b.version.editorial.cmp(&a.version.editorial)
    });

    println!("Found {} files", parsed_list.len());
    if parsed_list.is_empty() {
        return;
    }
    if args.list {
        for entry in parsed_list.iter() {
            println!("{}", entry);
        }
    } else {
        let latest_file = parsed_list.first().unwrap();
        println!("Downloading {}...", latest_file.name);
        let data = ftp_stream
            .retr_as_buffer(
                format!(
                    "Specs/archive/{series}_series/{}/{}",
                    args.spec, latest_file.name
                )
                .as_str(),
            )
            .expect("Failed to download");
        let mut file = fs::File::create(latest_file.name)
            .expect(format!("Failed to create file {}", latest_file.name).as_str());
        file.write_all(&data.into_inner()).unwrap();
        println!("Done");
    }
    let _ = ftp_stream.quit().unwrap();
}
