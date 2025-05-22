use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

/// Parse a YAML file into a Yaml value
pub fn parse_file(path: &Path) -> Result<Vec<Yaml>> {
    let content = fs::read_to_string(path)
        .context(format!("Failed to read YAML file: {}", path.display()))?;

    parse_string(&content)
}

/// Parse a YAML string into a Yaml value
pub fn parse_string(content: &str) -> Result<Vec<Yaml>> {
    YamlLoader::load_from_str(content).context("Failed to parse YAML content")
}

/// Convert a Yaml value to a string
pub fn to_string(yaml: &Yaml) -> Result<String> {
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);

    emitter.dump(yaml).context("Failed to emit YAML")?;

    Ok(out_str)
}

/// Write a Yaml value to a file
pub fn write_file(yaml: &Yaml, path: &Path) -> Result<()> {
    let yaml_str = to_string(yaml)?;

    fs::write(path, yaml_str)
        .context(format!("Failed to write YAML to file: {}", path.display()))?;

    Ok(())
}
