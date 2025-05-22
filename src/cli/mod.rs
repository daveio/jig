use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A tool to manage various aspects of a development environment
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Run in dry-run mode (only explain what would be changed)
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new repository with the specified language
    New(NewArgs),

    /// Update files in a repository from a potentially changed template set
    Update(UpdateArgs),

    /// Set up AI support in tools
    Ai(AiArgs),

    /// Bump versions in package managers and configuration files
    Bump(BumpArgs),
}

#[derive(clap::Args, Debug)]
pub struct NewArgs {
    /// Programming language to use
    pub language: String,
}

#[derive(clap::Args, Debug)]
pub struct UpdateArgs {
    /// Repository to update (uses current directory if not specified)
    pub repository: Option<PathBuf>,
}

#[derive(clap::Args, Debug)]
pub struct AiArgs {
    /// Tool to configure (configures all tools if not specified)
    pub tool: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct BumpArgs {
    /// Repository to update versions in (uses current directory if not specified)
    pub repository: Option<PathBuf>,
}
