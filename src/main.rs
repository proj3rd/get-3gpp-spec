use chrono::{DateTime, Months};
use clap::Parser;
mod http;
use http::get;
mod numbering;
mod parsed_file;
mod versioning;

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

    get(args.spec, args.rel, start_date, end_date, args.list);
}
