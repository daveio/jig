use crate::error::Result;
use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    /// Overwrite existing config (with a new key!)
    #[arg(short = 'c', long = "clobber")]
    clobber: bool,
}

pub fn execute(args: InitArgs) -> Result<()> {
    println!("jig init: Not implemented yet");
    if args.clobber {
        println!("  --clobber flag provided");
    }
    Ok(())
}
