use crate::cli::CommonOptions;
use anyhow::Result;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Write content to stdout or a file
pub fn write_output(content: &str, options: &CommonOptions) -> Result<()> {
    match &options.output {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            if options.verbose {
                println!("📄 Output written to: {}", path.display());
            }
        }
        None => {
            print!("{}", content);
            io::stdout().flush()?;
        }
    }
    Ok(())
}

/// Format content for AI consumption
pub fn format_for_ai(title: &str, content: &str, command_hint: &str) -> String {
    format!(
        "# {}\n\n{}\n\n## Suggested Command\n\n```bash\n{}\n```\n",
        title, content, command_hint
    )
}

/// Print a message with optional AI formatting and file output
pub fn print_message(
    message: &str,
    options: &CommonOptions,
    title: Option<&str>,
    command_hint: Option<&str>,
) -> Result<()> {
    let content = if options.ai && options.info {
        match (title, command_hint) {
            (Some(t), Some(cmd)) => format_for_ai(t, message, cmd),
            _ => format!("# Output\n\n{}\n", message),
        }
    } else {
        message.to_string()
    };

    write_output(&content, options)
}
