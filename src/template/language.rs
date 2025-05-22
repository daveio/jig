use anyhow::Result;
use log::debug;
use std::path::Path;
use walkdir::WalkDir;

/// Detect the programming language of a repository
pub fn detect_language(repo_path: &Path) -> Result<String> {
    debug!(
        "Detecting language for repository at: {}",
        repo_path.display()
    );

    // Language indicators with their corresponding file patterns
    let language_indicators = [
        (
            "rust",
            vec![
                "Cargo.toml",
                "Cargo.lock",
                ".rs",
                "rust-toolchain",
                "rust-toolchain.toml",
            ],
        ),
        (
            "python",
            vec![
                "pyproject.toml",
                "setup.py",
                "requirements.txt",
                ".py",
                "Pipfile",
                "poetry.lock",
            ],
        ),
        (
            "javascript",
            vec![
                "package.json",
                "package-lock.json",
                ".js",
                ".jsx",
                "yarn.lock",
                ".npmrc",
            ],
        ),
        (
            "typescript",
            vec![
                "tsconfig.json",
                ".ts",
                ".tsx",
                "tslint.json",
                "tsconfig.*.json",
            ],
        ),
        (
            "go",
            vec!["go.mod", "go.sum", ".go", "Gopkg.toml", "Gopkg.lock"],
        ),
        (
            "java",
            vec![
                "pom.xml",
                "build.gradle",
                ".java",
                ".gradle",
                "gradle.properties",
                "settings.gradle",
            ],
        ),
        (
            "ruby",
            vec!["Gemfile", "Gemfile.lock", ".rb", ".gemspec", "Rakefile"],
        ),
        (
            "php",
            vec!["composer.json", "composer.lock", ".php", "artisan"],
        ),
        (
            "csharp",
            vec![".csproj", ".sln", ".cs", "packages.config", "NuGet.Config"],
        ),
        (
            "cpp",
            vec![
                "CMakeLists.txt",
                ".cpp",
                ".hpp",
                ".cc",
                ".hh",
                ".cxx",
                ".hxx",
            ],
        ),
        (
            "c",
            vec![
                ".c",
                ".h",
                "Makefile",
                "configure",
                "configure.ac",
                "autoconf",
            ],
        ),
        ("shell", vec![".sh", ".bash", ".zsh", ".fish"]),
    ];

    // Count occurrences of each language's indicators
    let mut language_counts = std::collections::HashMap::new();

    // Walk through the repository
    for entry in WalkDir::new(repo_path).max_depth(5) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_lowercase(),
            None => continue,
        };

        let extension = match path.extension() {
            Some(ext) => format!(".{}", ext.to_string_lossy().to_lowercase()),
            None => String::new(),
        };

        // Check each language's indicators
        for (lang, indicators) in &language_indicators {
            for indicator in indicators {
                let indicator = indicator.to_lowercase();
                if file_name == indicator || extension == indicator {
                    *language_counts.entry(*lang).or_insert(0) += 1;
                    break;
                }
            }
        }
    }

    // Find the language with the most indicators
    let detected_language = language_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(lang, _)| lang.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    debug!("Detected language: {}", detected_language);

    Ok(detected_language)
}

/// Get a list of supported languages
pub fn get_supported_languages() -> Vec<String> {
    vec![
        "rust".to_string(),
        "python".to_string(),
        "javascript".to_string(),
        "typescript".to_string(),
        "go".to_string(),
        "java".to_string(),
        "ruby".to_string(),
        "php".to_string(),
        "csharp".to_string(),
        "cpp".to_string(),
        "c".to_string(),
        "shell".to_string(),
    ]
}

/// Check if a language is supported
pub fn is_language_supported(language: &str) -> bool {
    get_supported_languages().contains(&language.to_lowercase())
}
