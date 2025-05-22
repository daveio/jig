use clap::Parser;
use jig::cli::{Cli, Commands};
use jig::config::ConfigManager;
use jig::utils::logging;
use std::process;

fn main() {
    // Initialize logging
    logging::init_logger();

    // Initialize configuration if it doesn't exist
    // This ensures config files are created even when --help is called
    if let Err(e) = ConfigManager::initialize_if_not_exists() {
        eprintln!("Error initializing configuration: {}", e);
        process::exit(1);
    }

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
