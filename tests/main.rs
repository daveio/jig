use assert_cmd::Command;
use predicates::str::contains;

/// Tests that the long help output contains the long about text.
#[test]
fn long_help_shows_long_about() {
    let mut cmd = Command::cargo_bin("jig").unwrap();
    cmd.arg("--help").assert().success().stdout(contains(
        r#"jig: a collection of wonderful things

jig is a collection of CLI tools and utilities which I've 
wanted to have available at the command line at some point.
It's fairly chaotic, and I encourage you to embrace the chaos.

If you find something particularly useful, or have ideas for
features or adjustments, please let me know!

-- Dave Williams <dave@dave.io> <https://dave.io>"#,
    ));
}

/// Tests that the short help output contains the short about text.
#[test]
fn short_help_shows_short_about() {
    let mut cmd = Command::cargo_bin("jig").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(contains("jig: a collection of wonderful things"));
}
