use crate::ai;
use crate::cli::{AiArgs, CommonOptions};
use anyhow::{Context, Result};
use log::info;

/// Execute the 'ai' command
pub fn execute(args: &AiArgs, options: &CommonOptions) -> Result<()> {
    match &args.tool {
        Some(tool) => {
            if options.verbose {
                println!("🤖 Setting up AI support for tool: {}", tool);
            }
            info!("Setting up AI support for tool: {}", tool);

            if options.info {
                println!("ℹ️  Would configure AI support for {}", tool);
                if options.ai {
                    println!("# AI Tool Configuration\n\nTool: {}\nAction: Configure AI support for specified tool", tool);
                }
                return Ok(());
            }

            if options.dry_run {
                println!("🔍 [DRY RUN] Would configure AI support for {}", tool);
                info!("Dry run mode: No changes will be made");
                return Ok(());
            }

            ai::configure_tool(tool)
                .context(format!("Failed to configure AI support for {}", tool))?;

            if options.verbose {
                println!("✅ AI support configured successfully for {}!", tool);
            }
            info!("AI support configured successfully for {}", tool);
        }
        None => {
            if options.verbose {
                println!("🤖 Setting up AI support for all tools");
            }
            info!("Setting up AI support for all tools");

            if options.info {
                println!("ℹ️  Would configure AI support for all tools");
                if options.ai {
                    println!("# AI Tools Configuration\n\nAction: Configure AI support for all detected tools");
                }
                return Ok(());
            }

            if options.dry_run {
                println!("🔍 [DRY RUN] Would configure AI support for all tools");
                info!("Dry run mode: No changes will be made");
                return Ok(());
            }

            ai::configure_all_tools().context("Failed to configure AI support for all tools")?;

            if options.verbose {
                println!("✅ AI support configured successfully for all tools!");
            }
            info!("AI support configured successfully for all tools");
        }
    }

    Ok(())
}
