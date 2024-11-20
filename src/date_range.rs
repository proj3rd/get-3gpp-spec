use chrono::{DateTime, FixedOffset, Months};

pub struct DateRange {
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
}

pub fn parse_date_range(maybe_date: Option<String>) -> Result<Option<DateRange>, String> {
    let yyyy_mm = match maybe_date {
        None => return Ok(None),
        Some(str) => str,
    };
    let start_date = match DateTime::parse_from_str(
        format!("{yyyy_mm}-01 00:00:00 +0000").as_str(),
        "%Y-%m-%d %H:%M:%S %z",
    ) {
        Ok(parsed_date) => parsed_date,
        Err(_) => {
            return Err(format!(
                "Date must be in a form of YYYY-MM, but got {yyyy_mm}"
            ))
        }
    };
    let end_date = start_date
        .clone()
        .checked_add_months(Months::new(3))
        .unwrap();
    Ok(Some(DateRange {
        start_date,
        end_date,
    }))
}
