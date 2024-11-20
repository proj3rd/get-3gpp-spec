use clap::Parser;
mod date_range;
mod ftp;
mod http;
use date_range::parse_date_range;
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
    let maybe_date_range = match parse_date_range(args.date) {
        Ok(date_range) => date_range,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    http::get(&args.spec, args.rel, &maybe_date_range, args.list);
    // ftp::get(&args.spec, args.rel, &maybe_date_range, args.list);
}
