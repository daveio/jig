use clap::Parser;
use jig::cli::{Cli, Commands, CommonOptions};
use jig::config::ConfigManager;
use jig::utils::logging;
use std::process;

fn main() {
    // Initialize logging
    logging::init_logger();

    // Initialize configuration if it doesn't exist
    // This ensures config files are created even when --help is called
    if let Err(e) = ConfigManager::initialize_if_not_exists() {
        eprintln!("❌ Error initializing configuration: {}", e);
        process::exit(1);
    }

    // Parse command line arguments
    let cli = Cli::parse();

    // Create common options from CLI args
    let options = CommonOptions::from(&cli);

    // Validate flag combinations
    if options.ai && !options.info {
        eprintln!("❌ Error: --ai flag requires --info flag");
        process::exit(1);
    }

    // Execute the command
    let result = match &cli.command {
        Commands::New(args) => jig::commands::new::execute(args, &options),
        Commands::Update(args) => jig::commands::update::execute(args, &options),
        Commands::Ai(args) => jig::commands::ai::execute(args, &options),
        Commands::Bump(args) => jig::commands::bump::execute(args, &options),
        Commands::Dependabot(args) => jig::commands::dependabot::execute(args, &options),
    };

    // Handle errors
    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}
