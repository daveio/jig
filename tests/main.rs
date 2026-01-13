use assert_cmd::cargo;
use predicates::str::contains;

// Import constants from the symlinked constants file
#[path = "constants.rs"]
mod constants;
use constants::{LONG_ABOUT_TEST, SHORT_ABOUT_TEST};

/// Tests that the long help output contains the long about text.
#[test]
fn long_help_shows_long_about() {
    let mut cmd = cargo::cargo_bin_cmd!("jig");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains(LONG_ABOUT_TEST));
}

/// Tests that the short help output contains the short about text.
#[test]
fn short_help_shows_short_about() {
    let mut cmd = cargo::cargo_bin_cmd!("jig");
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(contains(SHORT_ABOUT_TEST));
}

/// Tests that the application shows version information.
#[test]
fn version_flag_works() {
    let mut cmd = cargo::cargo_bin_cmd!("jig");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(contains("jig 0.0.0-dev"));
}

/// Tests that the dry-run flag is accepted.
#[test]
fn dry_run_flag_works() {
    let mut cmd = cargo::cargo_bin_cmd!("jig");
    cmd.arg("--dry-run").assert().success();
}

/// Tests that the verbose flag is accepted.
#[test]
fn verbose_flag_works() {
    let mut cmd = cargo::cargo_bin_cmd!("jig");
    cmd.arg("--verbose").assert().success();
}

/// Tests that multiple verbose flags work.
#[test]
fn multiple_verbose_flags_work() {
    let mut cmd = cargo::cargo_bin_cmd!("jig");
    cmd.arg("-vvv").assert().success();
}
