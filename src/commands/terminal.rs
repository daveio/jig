use crate::error::Result;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum TerminalCommand {
    /// Display XKCD comics in terminal
    Xkcd(XkcdArgs),
}

#[derive(Args)]
pub struct XkcdArgs {
    /// Comic number (latest if not specified)
    number: Option<u32>,

    /// Show random comic
    #[arg(short = 'r', long = "random")]
    random: bool,
}

pub fn execute(command: TerminalCommand) -> Result<()> {
    match command {
        TerminalCommand::Xkcd(args) => {
            println!("jig terminal xkcd: Not implemented yet");
            if let Some(number) = args.number {
                println!("  Comic: {}", number);
            }
            if args.random {
                println!("  Random comic requested");
            }
        }
    }
    Ok(())
}
