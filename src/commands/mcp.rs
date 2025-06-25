use crate::error::Result;
use clap::Args;

#[derive(Args)]
pub struct McpArgs {
    /// Port to run MCP server on
    #[arg(short = 'p', long = "port")]
    port: Option<u16>,

    /// Enable debug mode
    #[arg(short = 'd', long = "debug")]
    debug: bool,
}

pub fn execute(args: McpArgs) -> Result<()> {
    println!("jig mcp: Not implemented yet");
    if let Some(port) = args.port {
        println!("  Port: {}", port);
    }
    if args.debug {
        println!("  Debug mode enabled");
    }
    Ok(())
}
