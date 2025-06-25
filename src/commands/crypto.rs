use crate::error::Result;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum CryptoCommand {
    /// Encrypt data using age encryption
    Encrypt(EncryptArgs),
    /// Decrypt data using age encryption
    Decrypt(DecryptArgs),
    /// Print the public key associated with a private key
    Public(PublicArgs),
}

#[derive(Args)]
pub struct EncryptArgs {
    /// File of plaintext to read. May be binary
    #[arg(short = 'i', long = "input")]
    input: Option<String>,

    /// File of ciphertext to write
    #[arg(short = 'o', long = "output")]
    output: Option<String>,

    /// Override key from configuration or env
    #[arg(short = 'k', long = "key")]
    key: Option<String>,
}

#[derive(Args)]
pub struct DecryptArgs {
    /// File of ciphertext to read
    #[arg(short = 'i', long = "input")]
    input: Option<String>,

    /// File of plaintext to write. May be binary
    #[arg(short = 'o', long = "output")]
    output: Option<String>,

    /// Override key from configuration or env
    #[arg(short = 'k', long = "key")]
    key: Option<String>,
}

#[derive(Args)]
pub struct PublicArgs {
    /// Private key to process
    #[arg(short = 'k', long = "key")]
    key: Option<String>,
}

pub fn execute(command: CryptoCommand) -> Result<()> {
    match command {
        CryptoCommand::Encrypt(args) => {
            println!("jig crypto encrypt: Not implemented yet");
            if let Some(input) = args.input {
                println!("  Input: {}", input);
            }
            if let Some(output) = args.output {
                println!("  Output: {}", output);
            }
        }
        CryptoCommand::Decrypt(args) => {
            println!("jig crypto decrypt: Not implemented yet");
            if let Some(input) = args.input {
                println!("  Input: {}", input);
            }
            if let Some(output) = args.output {
                println!("  Output: {}", output);
            }
        }
        CryptoCommand::Public(args) => {
            println!("jig crypto public: Not implemented yet");
            if let Some(key) = args.key {
                println!("  Key: {}", key);
            }
        }
    }
    Ok(())
}
