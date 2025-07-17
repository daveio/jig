use clap::Parser;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(
    about = "Multipurpose CLI toolbox",
    author = "Dave Williams <dave@dave.io>",
    long_about = "A collection of wonderful things. Tools and utilities which I find useful.",
    name = "jig",
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
