//  use pretty_assertions::assert_eq;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Hubbit",
    long_about = "Manage GitHub repositories and release binaries"
)]
struct Cli {
    /// The GitHub repository URL to process
    #[arg(required = true)]
    url: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Processing repository: {}", cli.url);
}
