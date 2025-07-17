mod util;

use anyhow::Result;
use clap::Parser;
use log::{debug, info};

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

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    let args = Cli::parse();

    debug!("Parsed CLI arguments: {args:?}");
    info!("Dry run mode: {}", args.dry_run);
    info!("Verbosity level: {}", args.verbose);

    Ok(())
}
