use std::{
    cmp::Ordering,
    fs::File,
    io::{self, Cursor},
    time::Duration,
};

use chrono::DateTime;
use regex::Regex;

use crate::{
    date_range::DateRange, numbering::get_series, parsed_file::ParsedFile,
    versioning::parse_version,
};

pub fn get(spec: &str, rel: Option<u8>, maybe_date_range: &Option<DateRange>, show_list: bool) {
    let series = get_series(&spec).unwrap();
    let path = format!("https://www.3gpp.org/ftp/Specs/archive/{series}_series/{spec}",);
    let response = reqwest::blocking::get(path).unwrap();
    assert!(response.status().is_success());
    let body = response.text().unwrap();

    let path_pattern = Regex::new(r"https.+?zip").unwrap();
    let datetime_pattern = Regex::new(r"\d{4}\/\d{2}\/\d{2} \d{1,2}:\d{2}").unwrap();
    let mut list: Vec<ParsedFile> = Vec::new();
    let mut start = 0;
    loop {
        match path_pattern.find_at(&body, start) {
            Some(path_match) => {
                let path = path_match.as_str();
                let last_index_of_slash = path.rfind("/").unwrap();
                let name = &path[last_index_of_slash + 1..];
                let last_index_of_hyphen = name.rfind("-").unwrap();
                let last_index_of_dot = name.rfind(".").unwrap();
                let version_string = &name[last_index_of_hyphen + 1..last_index_of_dot];
                let version = parse_version(version_string).unwrap();
                let date_time_string = datetime_pattern
                    .find_at(&body, path_match.end())
                    .unwrap()
                    .as_str();
                let date_time = DateTime::parse_from_str(
                    &format!("{date_time_string} +0000"),
                    "%Y/%m/%d%_H:%M %z",
                )
                .unwrap()
                .to_utc();
                list.push(ParsedFile {
                    name,
                    version,
                    date_time,
                });
                start = path_match.end();
            }
            None => break,
        }
    }
    let mut filtered_list = list
        .iter()
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
            };
            true
        })
        .collect::<Vec<&ParsedFile>>();
    filtered_list.sort_by(|a, b| {
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

    println!("Found {} files", filtered_list.len());
    if filtered_list.is_empty() {
        return;
    }
    if show_list {
        for entry in filtered_list.iter() {
            println!("{}", entry);
        }
    } else {
        let latest_file = filtered_list.first().unwrap();
        println!("Downloading {}...", latest_file.name);
        let timeout_secs = 60;
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .unwrap();
        let path = format!(
            "https://www.3gpp.org/ftp/Specs/archive/{series}_series/{spec}/{}",
            latest_file.name
        );
        let timeout_msg = format!(
            "{} {} {} {}",
            "Failed to download.",
            "Maybe the file is too large.",
            "You can download it from:",
            path
        );
        let response = client.get(&path).send().expect(&timeout_msg);
        let mut file = File::create(latest_file.name)
            .expect(format!("Failed to create file {}", latest_file.name).as_str());
        let mut content = Cursor::new(response.bytes().expect(&timeout_msg));
        io::copy(&mut content, &mut file).unwrap();
        println!("Done");
    }
}
