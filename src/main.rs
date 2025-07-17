mod util;

use clap::Parser;
use std::process::exit;
use util::constants::{AUTHOR, LONG_ABOUT, NAME, SHORT_ABOUT};

#[derive(Parser, Debug)]
#[command(
    about = SHORT_ABOUT,
    author = AUTHOR,
    long_about = LONG_ABOUT,
    name = NAME,
    version
)]
struct Cli {
    /// Run without making changes
    #[arg(short, long)]
    dry_run: bool,

    /// Verbosity level (can be specified multiple times)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Cli::parse();
    println!("Dry run: {}", args.dry_run);
    println!("Verbosity level: {}", args.verbose);
    exit(0); // Exit with success
}
