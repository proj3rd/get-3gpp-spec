use crate::versioning::Version;
use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(Debug)]
pub struct ParsedFile<'a> {
    pub name: &'a str,
    pub version: Version,
    pub date_time: DateTime<Utc>,
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
