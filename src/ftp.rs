use crate::{
    date_range::DateRange, numbering::get_series, parsed_file::ParsedFile,
    versioning::parse_version,
};
use chrono::{DateTime, Utc};
use std::{cmp::Ordering, fs, io::Write, str::FromStr};
use suppaftp::{list::File, FtpStream};

pub fn get(spec: &str, rel: Option<u8>, maybe_date_range: &Option<DateRange>, show_list: bool) {
    let series = get_series(&spec).unwrap();
    let path = format!("Specs/archive/{series}_series/{}", spec);
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
            match rel {
                Some(rel) => {
                    if rel != file.version.major {
                        return false;
                    }
                }
                None => {}
            };
            match maybe_date_range {
                None => {}
                Some(date_range) => {
                    if date_range.start_date > file.date_time
                        || date_range.end_date < file.date_time
                    {
                        return false;
                    }
                }
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
    if show_list {
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
                    spec, latest_file.name
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
