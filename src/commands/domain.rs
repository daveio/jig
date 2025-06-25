use crate::error::Result;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum DomainCommand {
    /// Check domain expiration dates
    Expiry(ExpiryArgs),
    /// Perform WHOIS lookups
    Whois(WhoisArgs),
    /// Check nameserver information
    Ns(NsArgs),
    /// Check domain availability
    Available(AvailableArgs),
}

#[derive(Args)]
pub struct ExpiryArgs {
    /// Domain to check
    domain: String,
}

#[derive(Args)]
pub struct WhoisArgs {
    /// Domain to lookup
    domain: String,
}

#[derive(Args)]
pub struct NsArgs {
    /// Domain to check
    domain: String,
}

#[derive(Args)]
pub struct AvailableArgs {
    /// Domain to check
    domain: String,
}

pub fn execute(command: DomainCommand) -> Result<()> {
    match command {
        DomainCommand::Expiry(args) => {
            println!("jig domain expiry: Not implemented yet");
            println!("  Domain: {}", args.domain);
        }
        DomainCommand::Whois(args) => {
            println!("jig domain whois: Not implemented yet");
            println!("  Domain: {}", args.domain);
        }
        DomainCommand::Ns(args) => {
            println!("jig domain ns: Not implemented yet");
            println!("  Domain: {}", args.domain);
        }
        DomainCommand::Available(args) => {
            println!("jig domain available: Not implemented yet");
            println!("  Domain: {}", args.domain);
        }
    }
    Ok(())
}
