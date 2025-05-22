use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// 🛠️ A tool to manage various aspects of a development environment
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 🔍 Run in dry-run mode (only explain what would be changed)
    #[arg(long, short = 'd', global = true)]
    pub dry_run: bool,

    /// ℹ️ Show information about changes without making them
    #[arg(long, short = 'i', global = true)]
    pub info: bool,

    /// 📝 Show detailed information during execution
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// 🤖 Format output for AI consumption (requires --info)
    #[arg(long, short = 'a', global = true)]
    pub ai: bool,

    /// 📄 Redirect output to a file
    #[arg(long, short = 'o', global = true)]
    pub output: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 🆕 Create a new repository with the specified language
    New(NewArgs),

    /// 🔄 Update files in a repository from a potentially changed template set
    Update(UpdateArgs),

    /// 🤖 Set up AI support in tools
    Ai(AiArgs),

    /// ⬆️ Bump versions in package managers and configuration files
    Bump(BumpArgs),

    /// 🤖 Manage Dependabot configuration
    Dependabot(DependabotArgs),
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

    /// Use cached versions only (don't check for updates online)
    #[arg(long, short)]
    pub cached: bool,

    /// Ecosystem to target (if not specified, targets all detected ecosystems)
    #[command(subcommand)]
    pub ecosystem: Option<BumpEcosystem>,
}

#[derive(Subcommand, Debug)]
pub enum BumpEcosystem {
    /// 📦 Update Node.js/npm packages and versions
    Node,

    /// 🐍 Update Python packages and versions
    Python,

    /// 💎 Update Ruby packages and versions
    Ruby,

    /// 🦀 Update Rust packages and versions
    Rust,

    /// ☕ Update Java packages and versions
    Java,

    /// 🐹 Update Go packages and versions
    Go,

    /// ⚙️ Update GitHub Actions workflows
    Actions,
}

#[derive(clap::Args, Debug)]
pub struct DependabotArgs {
    /// Repository to update dependabot configuration in (uses current directory if not specified)
    pub repository: Option<PathBuf>,
}

/// Common options shared across commands
#[derive(Debug, Clone)]
pub struct CommonOptions {
    pub dry_run: bool,
    pub info: bool,
    pub verbose: bool,
    pub ai: bool,
    pub output: Option<PathBuf>,
}

impl From<&Cli> for CommonOptions {
    fn from(cli: &Cli) -> Self {
        Self {
            dry_run: cli.dry_run,
            info: cli.info,
            verbose: cli.verbose,
            ai: cli.ai,
            output: cli.output.clone(),
        }
    }
}
