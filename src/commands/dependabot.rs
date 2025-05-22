use anyhow::Result;
use std::path::Path;

use crate::cli::{CommonOptions, DependabotArgs};
use crate::dependabot::{detect_ecosystems, find_dependabot_config, update_dependabot_config};

/// Execute the dependabot command
pub fn execute(args: &DependabotArgs, options: &CommonOptions) -> Result<()> {
    let repo_path = args.repository.as_deref().unwrap_or_else(|| Path::new("."));

    if options.verbose {
        println!(
            "🤖 Managing Dependabot configuration in {}",
            repo_path.display()
        );
    }

    // Detect ecosystems first for info mode
    let detected_ecosystems = detect_ecosystems(repo_path)?;
    let existing_config = find_dependabot_config(repo_path);

    if options.info {
        let config_status = if existing_config.is_some() {
            "Found existing dependabot configuration"
        } else {
            "No dependabot configuration found - would create new one"
        };

        let mut info_message = format!("ℹ️  {}\n", config_status);

        if !detected_ecosystems.is_empty() {
            info_message.push_str(&format!(
                "🔍 Detected ecosystems: {}\n",
                detected_ecosystems
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        if options.ai {
            let mut ai_content = "# Dependabot Configuration Analysis\n\n".to_string();
            ai_content.push_str(&format!("Repository: {}\n", repo_path.display()));
            ai_content.push_str(&format!("Status: {}\n", config_status));

            if !detected_ecosystems.is_empty() {
                ai_content.push_str("Detected ecosystems:\n");
                for ecosystem in &detected_ecosystems {
                    ai_content.push_str(&format!("- {}\n", ecosystem));
                }
            }

            ai_content.push_str("\n## Suggested Command\n\n```bash\njig dependabot\n```\n");
            crate::utils::output::write_output(&ai_content, options)?;
        } else {
            crate::utils::output::write_output(&info_message, options)?;
        }
        return Ok(());
    }

    if options.dry_run {
        println!(
            "🔍 [DRY RUN] Would update Dependabot configuration for ecosystems: {}",
            detected_ecosystems
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Ok(());
    }

    // Default assignee (you can make this configurable later)
    let assignees = vec!["daveio".to_string()];

    // Update dependabot configuration
    let added_ecosystems = update_dependabot_config(repo_path, false, assignees)?;

    if added_ecosystems.is_empty() {
        if options.verbose {
            println!("✅ Dependabot configuration is already up to date!");
        }
        println!("🤖 No new ecosystems to add to Dependabot configuration");
    } else if options.verbose {
        println!(
            "✅ Added {} ecosystems to Dependabot configuration",
            added_ecosystems.len()
        );
        for ecosystem in &added_ecosystems {
            println!("  📦 {}", ecosystem);
        }
    } else {
        println!(
            "🤖 Added {} ecosystems to Dependabot: {}",
            added_ecosystems.len(),
            added_ecosystems.join(", ")
        );
    }

    Ok(())
}
