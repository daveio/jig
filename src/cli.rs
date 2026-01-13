use crate::util::constants::{LONG_ABOUT, SHORT_ABOUT};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jig")]
#[command(about = SHORT_ABOUT)]
#[command(long_about = LONG_ABOUT)]
#[command(version)]
pub struct Cli {
    /// Verbose output (can be repeated: -v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Dry run mode
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Cryptographic operations
    Crypto {
        #[command(subcommand)]
        command: CryptoCommand,
    },
    /// Generate secure values
    Generate {
        #[command(subcommand)]
        command: GenerateCommand,
    },
}

#[derive(Subcommand)]
pub enum CryptoCommand {
    /// Encrypt a file using age
    Encrypt {
        /// File to encrypt
        file: String,
    },
    /// Decrypt a file using age
    Decrypt {
        /// File to decrypt
        file: String,
    },
}

#[derive(Subcommand)]
pub enum GenerateCommand {
    /// Generate a secure password
    Password {
        /// Length of password
        #[arg(short, long, default_value = "32")]
        length: usize,
    },
}
