// MEMO: テストはデフォルトで並列実行なので、Dockerコンテナを使うテストは別名をつけること
use assert_cmd::Command;
use predicates::str::contains;
use std::env;
use std::process::Command as StdCommand;
use std::{thread, time};

fn wait_for_container_removal(container_name: &str) {
    for _ in 0..5 {
        let output = StdCommand::new("sh")
            .arg("-c")
            .arg(format!(
                "docker ps -a --format '{{.Names}}' | grep {}",
                container_name
            ))
            .output()
            .expect("Failed to check running containers");

        if output.stdout.is_empty() {
            return;
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}

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
    output.stdout(contains("No running containers found."));
}

#[test]
fn test_dosukoi_with_running_containers() {
    let container_name = "test_container";

    StdCommand::new("docker")
        .args(["rm", "-f", container_name])
        .status()
        .ok();

    wait_for_container_removal(container_name);

    let status = StdCommand::new("docker")
        .args([
            "run",
            "-d",
            "--name",
            container_name,
            "alpine",
            "sleep",
            "60",
        ])
        .status()
        .expect("Failed to start test container");

    assert!(status.success(), "Failed to start test container");

    let ps_output = StdCommand::new("sh")
        .arg("-c")
        .arg("docker ps --format '{{.Names}}'")
        .output()
        .expect("Failed to execute docker ps");

    let ps_stdout = String::from_utf8_lossy(&ps_output.stdout);
    assert!(
        ps_stdout.contains(container_name),
        "Container not found in `docker ps`"
    );

    let mut cmd = Command::cargo_bin("dosukoi").unwrap();
    cmd.assert()
        .stdout(contains("Selected containers dosukoi!"));

    StdCommand::new("docker")
        .args(["rm", "-f", container_name])
        .status()
        .expect("Failed to remove test container");

    wait_for_container_removal(container_name);
}

#[test]
fn test_dosukoi_with_project() {
    let container_name = "test_container_2";
    let project_name = "test_project_2";

    StdCommand::new("docker")
        .args(["rm", "-f", container_name])
        .status()
        .ok();
    wait_for_container_removal(container_name);

    let status = StdCommand::new("docker")
        .args([
            "run",
            "-d",
            "--name",
            container_name,
            "--label",
            &format!("com.docker.compose.project={}", project_name),
            "alpine",
            "sleep",
            "60",
        ])
        .status()
        .expect("Failed to start test container");

    assert!(status.success(), "Failed to start test container");

    let ps_output = StdCommand::new("sh")
        .arg("-c")
        .arg(&format!(
            "docker ps --filter 'label=com.docker.compose.project={}' --format '{{{{.Names}}}}'",
            project_name
        ))
        .output()
        .expect("Failed to execute docker ps");

    let ps_stdout = String::from_utf8_lossy(&ps_output.stdout);
    assert!(
        ps_stdout.contains(container_name),
        "Project container not found in `docker ps` output"
    );

    let mut cmd = Command::cargo_bin("dosukoi").unwrap();
    cmd.arg(project_name)
        .assert()
        .stdout(contains("Selected containers dosukoi!"));

    StdCommand::new("docker")
        .args(["rm", "-f", container_name])
        .status()
        .expect("Failed to remove test container");

    wait_for_container_removal(container_name);
}
