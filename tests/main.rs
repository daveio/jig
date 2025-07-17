use assert_cmd::Command;

/// This test checks if the `ls` command runs successfully
#[test]
fn ls_runs() {
    let mut cmd = Command::cargo_bin("jig").unwrap();
    cmd.assert().success();
}
