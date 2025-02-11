use assert_cmd::Command;
use std::env;
use std::process::Command as StdCommand;

#[test]
fn test_dosukoi_help() {
    let mut cmd = Command::cargo_bin("dosukoi").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn test_dosukoi_version() {
    let mut cmd = Command::cargo_bin("dosukoi").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_dosukoi_list() {
    env::set_var("DIALOGUER_NO_TTY", "1");

    let mut cmd = Command::cargo_bin("dosukoi").unwrap();
    cmd.arg("--ls").assert().success();
}

#[test]
fn test_dosukoi_no_containers() {
    let mut cmd = Command::cargo_bin("dosukoi").expect("Binary not found");

    let output = cmd.assert().success();

    output.stdout(predicates::str::contains("No running containers found."));
}

#[test]
fn test_dosukoi_with_running_containers() {
    // モックとしてdockerコンテナを作る
    StdCommand::new("docker")
        .args([
            "run",
            "-d",
            "--name",
            "test_container",
            "alpine",
            "sleep",
            "60",
        ])
        .status()
        .expect("Failed to start test container");

    let mut cmd = Command::cargo_bin("dosukoi").unwrap();
    cmd.assert()
        .stdout(predicates::str::contains("test_container"));

    StdCommand::new("docker")
        .args(["rm", "-f", "test_container"])
        .status()
        .expect("Failed to remove test container");
}
