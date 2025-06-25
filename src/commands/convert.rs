use crate::error::Result;
use clap::Args;

#[derive(Args)]
pub struct ConvertArgs {
    /// Input data to convert
    input: Option<String>,

    /// Input format
    #[arg(short = 'f', long = "from")]
    from: Option<String>,

    /// Output format
    #[arg(short = 't', long = "to")]
    to: Option<String>,
}

pub fn execute(args: ConvertArgs) -> Result<()> {
    println!("jig convert: Not implemented yet");
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
