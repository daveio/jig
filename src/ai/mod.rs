use anyhow::{Context, Result};
use log::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::paths;

/// Supported AI tools
pub enum AiTool {
    ClaudeDesktop,
    Cursor,
    Zed,
    Goose,
}

impl AiTool {
    /// Convert a string to an AiTool
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "claude" | "claudedesktop" | "claude-desktop" => Some(AiTool::ClaudeDesktop),
            "cursor" => Some(AiTool::Cursor),
            "zed" => Some(AiTool::Zed),
            "goose" => Some(AiTool::Goose),
            _ => None,
        }
    }

    /// Get the name of the tool
    pub fn name(&self) -> &'static str {
        match self {
            AiTool::ClaudeDesktop => "Claude Desktop",
            AiTool::Cursor => "Cursor",
            AiTool::Zed => "Zed",
            AiTool::Goose => "Goose",
        }
    }

    /// Get the config file path for the tool
    pub fn config_path(&self) -> Result<PathBuf> {
        let home_dir = dirs::home_dir().context("Failed to get home directory")?;

        let path = match self {
            AiTool::ClaudeDesktop => home_dir.join(".config/claude"),
            AiTool::Cursor => home_dir.join(".cursor"),
            AiTool::Zed => home_dir.join(".config/zed"),
            AiTool::Goose => home_dir.join(".config/goose"),
        };

        Ok(path)
    }
}

/// Configure AI support for a specific tool
pub fn configure_tool(tool_name: &str) -> Result<()> {
    let tool = AiTool::from_str(tool_name)
        .ok_or_else(|| anyhow::anyhow!("Unsupported AI tool: {}", tool_name))?;

    info!("Configuring AI support for {}", tool.name());

    // Get the baseline repository path
    let baseline_path = Path::new("/Users/dave/src/github.com/daveio/_baseline");

    if !baseline_path.exists() {
        anyhow::bail!("Baseline repository not found at {}", baseline_path.display());
    }

    match tool {
        AiTool::ClaudeDesktop => {
            let source_file = baseline_path.join("mcp-claude-desktop.json");
            let config_dir = tool.config_path()?;
            let target_file = config_dir.join("mcp.json");

            copy_config_file(&source_file, &target_file)?;

            info!("Claude Desktop configuration updated successfully");
        },
        AiTool::Cursor => {
            let source_dir = baseline_path.join("_cursor");
            let config_dir = tool.config_path()?;

            copy_directory(&source_dir, &config_dir)?;

            // Also copy MCP config
            let mcp_source = baseline_path.join("mcp-cursor.json");
            let mcp_target = config_dir.join("mcp.json");

            copy_config_file(&mcp_source, &mcp_target)?;

            info!("Cursor configuration updated successfully");
        },
        AiTool::Zed => {
            let source_dir = baseline_path.join(".zed");
            let config_dir = tool.config_path()?;

            copy_directory(&source_dir, &config_dir)?;

            // Also copy MCP config
            let mcp_source = baseline_path.join("mcp-zed.json");
            let mcp_target = config_dir.join("mcp.json");

            copy_config_file(&mcp_source, &mcp_target)?;

            info!("Zed configuration updated successfully");
        },
        AiTool::Goose => {
            let source_file = baseline_path.join("mcp-goose.yaml");
            let config_dir = tool.config_path()?;
            let target_file = config_dir.join("mcp.yaml");

            copy_config_file(&source_file, &target_file)?;

            info!("Goose configuration updated successfully");
        },
    }

    Ok(())
}

/// Configure AI support for all tools
pub fn configure_all_tools() -> Result<()> {
    let tools = vec![
        "claude-desktop",
        "cursor",
        "zed",
        "goose",
    ];

    for tool in tools {
        if let Err(e) = configure_tool(tool) {
            debug!("Failed to configure {}: {}", tool, e);
            // Continue with other tools even if one fails
        }
    }

    Ok(())
}

/// Copy a configuration file, creating directories if needed
fn copy_config_file(source: &Path, target: &Path) -> Result<()> {
    debug!("Copying config file from {} to {}", source.display(), target.display());

    if !source.exists() {
        anyhow::bail!("Source file does not exist: {}", source.display());
    }

    // Create target directory if it doesn't exist
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source, target)
        .context(format!("Failed to copy file from {} to {}", source.display(), target.display()))?;

    Ok(())
}

/// Copy a directory recursively
fn copy_directory(source: &Path, target: &Path) -> Result<()> {
    debug!("Copying directory from {} to {}", source.display(), target.display());

    if !source.exists() {
        anyhow::bail!("Source directory does not exist: {}", source.display());
    }

    // Create target directory if it doesn't exist
    fs::create_dir_all(target)?;

    let options = fs_extra::dir::CopyOptions::new()
        .overwrite(true)
        .content_only(true);

    fs_extra::dir::copy(source, target, &options)
        .context(format!("Failed to copy directory from {} to {}", source.display(), target.display()))?;

    Ok(())
}
