// util/constants.rs
// Shared help text used by the CLI and integration tests.
// Note: These are the exact strings used in the CLI definition.
// They may be wrapped in help output due to terminal width.

#[allow(dead_code)]
pub const SHORT_ABOUT: &str =
    "jig is a CLI toolbox that unifies crypto, generate, api, git, project, network, domain, workspace, terminal, and ai utilities.";

#[allow(dead_code)]
pub const LONG_ABOUT: &str = concat!(
    "jig is a comprehensive CLI toolbox built in Rust that consolidates various utilities into ",
    "a unified command-line interface. It follows a modular, command-group architecture ",
    "with deep shell integration, YAML-based configuration, template-driven project scaffolding, ",
    "and integrations for crypto, generation, external APIs, git/github, networking, domains, ",
    "workspace management, terminal visuals, and ai-powered automation."
);

// Shorter strings for testing help output (accounts for text wrapping)
#[allow(dead_code)]
pub const SHORT_ABOUT_TEST: &str = "CLI toolbox that unifies crypto, generate, api";
#[allow(dead_code)]
pub const LONG_ABOUT_TEST: &str = "comprehensive CLI toolbox built in Rust";
#[allow(dead_code)]
pub const VERSION: &str = "jig 0.0.0-dev";
