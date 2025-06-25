use crate::error::Result;
use clap::Args;

#[derive(Args)]
pub struct FormatArgs {
    /// Input data to format
    input: Option<String>,

    /// Input format (json, yaml, toml, etc.)
    #[arg(short = 'f', long = "from")]
    from: Option<String>,

    /// Output format (json, yaml, toml, etc.)
    #[arg(short = 't', long = "to")]
    to: Option<String>,
}

pub fn execute(args: FormatArgs) -> Result<()> {
    println!("jig format: Not implemented yet");
    if let Some(input) = args.input {
        println!("  Input: {}", input);
    }
    if let Some(from) = args.from {
        println!("  From: {}", from);
    }
    if let Some(to) = args.to {
        println!("  To: {}", to);
    }
    Ok(())
}
