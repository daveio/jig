use clap::Parser;
use jig::cli::{Cli, Commands};
use jig::utils::logging;
use std::process;

fn main() {
    // Initialize logging
    logging::init_logger();

    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the command
    let result = match &cli.command {
        Commands::New(args) => jig::commands::new::execute(args, cli.dry_run),
        Commands::Update(args) => jig::commands::update::execute(args, cli.dry_run),
        Commands::Ai(args) => jig::commands::ai::execute(args, cli.dry_run),
        Commands::Bump(args) => jig::commands::bump::execute(args, cli.dry_run),
    };

    // Handle errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
