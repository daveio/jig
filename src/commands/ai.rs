use crate::ai;
use crate::cli::AiArgs;
use anyhow::{Context, Result};
use log::info;

/// Execute the 'ai' command
pub fn execute(args: &AiArgs, dry_run: bool) -> Result<()> {
    match &args.tool {
        Some(tool) => {
            info!("Setting up AI support for tool: {}", tool);

            if dry_run {
                info!("Dry run mode: No changes will be made");
                info!("Would configure AI support for {}", tool);
                return Ok(());
            }

            ai::configure_tool(tool)
                .context(format!("Failed to configure AI support for {}", tool))?;

            info!("AI support configured successfully for {}", tool);
        }
        None => {
            info!("Setting up AI support for all tools");

            if dry_run {
                info!("Dry run mode: No changes will be made");
                info!("Would configure AI support for all tools");
                return Ok(());
            }

            ai::configure_all_tools().context("Failed to configure AI support for all tools")?;

            info!("AI support configured successfully for all tools");
        }
    }

    Ok(())
}
