mod cli;
mod util;

use clap::Parser;
use cli::Cli;

fn main() {
    let _cli = Cli::parse();
    // For now, if no subcommands are implemented and it hasn't exited via help/version,
    // we don't need to do anything else yet.
}
