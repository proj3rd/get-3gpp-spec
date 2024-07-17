use std::str::FromStr;

use chrono::{DateTime, Utc};
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

fn main() {
    let args = Args::parse();
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
    let parsed_list = list
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
        .collect::<Vec<ParsedFile>>();
    for entry in parsed_list.iter() {
        println!("{:?}", entry);
    }
    let _ = ftp_stream.quit().unwrap();
}
