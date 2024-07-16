use std::str::FromStr;

use suppaftp::list::File;
use suppaftp::FtpStream;
mod numbering;
use numbering::get_series;
mod versioning;
use versioning::{Version, parse_version};

#[derive(Debug)]
struct ParsedFile<'a> {
    name: &'a str,
    version: Version,
}

fn main() {
    let spec = "36.331"; // TODO: extract from args
    let series = get_series(spec).unwrap();
    let path = format!("Specs/archive/{series}_series/{spec}");
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
            ParsedFile { name, version }
        })
        .collect::<Vec<ParsedFile>>();
    for entry in parsed_list.iter() {
        println!("{:?}", entry);
    }
    let _ = ftp_stream.quit().unwrap();
}
