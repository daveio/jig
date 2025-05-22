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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use yaml_rust2::Yaml;

    #[test]
    fn test_parse_string_valid_yaml() {
        let yaml_content = r#"
name: test
version: 1.0.0
dependencies:
  - rust
  - serde
"#;

        let result = parse_string(yaml_content);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.len(), 1);

        let doc = &parsed[0];
        assert_eq!(doc["name"].as_str(), Some("test"));
        assert_eq!(doc["version"].as_str(), Some("1.0.0"));
        assert!(doc["dependencies"].as_vec().is_some());
    }

    #[test]
    fn test_parse_string_invalid_yaml() {
        let invalid_yaml = r#"
name: test
  invalid: - indentation
    - more problems
"#;

        let result = parse_string(invalid_yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_string_empty() {
        let result = parse_string("");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        // An empty string doesn't actually produce a document, it produces an empty list
        if parsed.is_empty() {
            // This is the correct behavior - empty string produces no documents
            assert_eq!(parsed.len(), 0);
        } else {
            // Or it might produce a single null document
            assert_eq!(parsed.len(), 1);
            assert!(parsed[0].is_null());
        }
    }

    #[test]
    fn test_to_string_simple_hash() {
        // Create YAML by parsing a string, then convert back to string
        let yaml_content = r#"name: test
version: 1.0.0"#;
        let parsed = parse_string(yaml_content).unwrap();
        let yaml = &parsed[0];

        let result = to_string(yaml);
        assert!(result.is_ok());

        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("name"));
        assert!(yaml_str.contains("test"));
        assert!(yaml_str.contains("version"));
        assert!(yaml_str.contains("1.0.0"));
    }

    #[test]
    fn test_to_string_array() {
        let array = vec![
            Yaml::String("rust".to_string()),
            Yaml::String("serde".to_string()),
            Yaml::String("tokio".to_string()),
        ];
        let yaml = Yaml::Array(array);

        let result = to_string(&yaml);
        assert!(result.is_ok());

        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("rust"));
        assert!(yaml_str.contains("serde"));
        assert!(yaml_str.contains("tokio"));
    }

    #[test]
    fn test_parse_file_valid() {
        let temp_dir = tempdir().unwrap();
        let yaml_path = temp_dir.path().join("test.yml");

        let yaml_content = r#"
name: test-project
language: rust
version: 0.1.0
"#;

        fs::write(&yaml_path, yaml_content).unwrap();

        let result = parse_file(&yaml_path);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["name"].as_str(), Some("test-project"));
        assert_eq!(parsed[0]["language"].as_str(), Some("rust"));
    }

    #[test]
    fn test_parse_file_nonexistent() {
        let nonexistent = Path::new("/this/file/does/not/exist.yml");
        let result = parse_file(nonexistent);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output.yml");

        // Create YAML by parsing a string
        let yaml_content = r#"project: jig
language: rust"#;
        let parsed = parse_string(yaml_content).unwrap();
        let yaml = &parsed[0];

        let result = write_file(yaml, &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());

        // Verify we can read it back
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("project"));
        assert!(content.contains("jig"));
        assert!(content.contains("language"));
        assert!(content.contains("rust"));
    }

    #[test]
    fn test_roundtrip_yaml() {
        let yaml_content = r#"
name: roundtrip-test
version: 2.0.0
features:
  - feature1
  - feature2
config:
  debug: true
  max_connections: 100
"#;

        // Parse the original
        let parsed = parse_string(yaml_content).unwrap();
        let original = &parsed[0];

        // Convert back to string
        let yaml_str = to_string(original).unwrap();

        // Parse the converted string
        let reparsed = parse_string(&yaml_str).unwrap();
        let roundtrip = &reparsed[0];

        // Compare key values
        assert_eq!(original["name"], roundtrip["name"]);
        assert_eq!(original["version"], roundtrip["version"]);
        assert_eq!(original["features"], roundtrip["features"]);
        assert_eq!(original["config"]["debug"], roundtrip["config"]["debug"]);
    }
}
