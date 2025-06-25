use clap::{Parser, Subcommand};

mod commands;
mod config;
mod error;
mod utils;

use crate::error::Result;

#[derive(Parser)]
#[command(name = "jig")]
#[command(about = "A CLI Toolbox", long_about = None)]
#[command(version)]
#[command(infer_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates initial config file and sets up shell integration
    Init(commands::init::InitArgs),

    /// Encryption and decryption operations
    Crypto {
        #[command(subcommand)]
        command: commands::crypto::CryptoCommand,
    },

    /// Generation utilities
    Generate {
        #[command(subcommand)]
        command: commands::generate::GenerateCommand,
    },

    /// Network utilities and diagnostics
    Network {
        #[command(subcommand)]
        command: commands::network::NetworkCommand,
    },

    /// Domain management and information tools
    Domain {
        #[command(subcommand)]
        command: commands::domain::DomainCommand,
    },

    /// TLS/SSL utilities and diagnostics
    Tls {
        #[command(subcommand)]
        command: commands::tls::TlsCommand,
    },

    /// Data formatting operations
    #[command(alias = "fmt")]
    Format(commands::format::FormatArgs),

    /// Data conversion utilities
    Convert(commands::convert::ConvertArgs),

    /// API operations and utilities
    Api {
        #[command(subcommand)]
        command: commands::api::ApiCommand,
    },

    /// Model Context Protocol server functionality
    Mcp(commands::mcp::McpArgs),

    /// Terminal utilities and enhancements
    Terminal {
        #[command(subcommand)]
        command: commands::terminal::TerminalCommand,
    },

    /// Project management utilities
    Project {
        #[command(subcommand)]
        command: commands::project::ProjectCommand,
    },

    /// Git utilities and enhancements
    Git {
        #[command(subcommand)]
        command: commands::git::GitCommand,
    },

    /// Workspace management and switching
    Workspace {
        #[command(subcommand)]
        command: commands::workspace::WorkspaceCommand,
    },

    /// AI-powered utilities
    Ai {
        #[command(subcommand)]
        command: commands::ai::AiCommand,
    },

    /// Easter egg command (hidden from help)
    #[command(hide = true)]
    Dance(commands::dance::DanceArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::execute(args),
        Commands::Crypto { command } => commands::crypto::execute(command),
        Commands::Generate { command } => commands::generate::execute(command),
        Commands::Network { command } => commands::network::execute(command),
        Commands::Domain { command } => commands::domain::execute(command),
        Commands::Tls { command } => commands::tls::execute(command),
        Commands::Format(args) => commands::format::execute(args),
        Commands::Convert(args) => commands::convert::execute(args),
        Commands::Api { command } => commands::api::execute(command),
        Commands::Mcp(args) => commands::mcp::execute(args),
        Commands::Terminal { command } => commands::terminal::execute(command),
        Commands::Project { command } => commands::project::execute(command),
        Commands::Git { command } => commands::git::execute(command),
        Commands::Workspace { command } => commands::workspace::execute(command),
        Commands::Ai { command } => commands::ai::execute(command),
        Commands::Dance(args) => commands::dance::execute(args),
    }
}
