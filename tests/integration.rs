use assert_cmd::Command;
use predicates::str::contains;
use std::env;
use std::process::Command as StdCommand; // 🔹 追加

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
    // ）
    StdCommand::new("docker")
        .args(["rm", "-f", "$(docker ps -aq)"])
        .status()
        .ok(); // なくても失敗は無視

    let mut cmd = Command::cargo_bin("dosukoi").expect("Binary not found");

    let output = cmd.assert().success();

    output.stdout(contains("No running containers found."));
}

#[test]
fn test_dosukoi_with_running_containers() {
    let _ = StdCommand::new("docker")
        .args(["rm", "-f", "test_container"])
        .status()
        .ok(); // なくても失敗は無視

    // 2️⃣ テスト用のコンテナを作成
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
        .stdout(predicates::str::contains("Selected containers dosukoi!"));

    // 4️⃣ コンテナを削除
    let status = StdCommand::new("docker")
        .args(["rm", "-f", "test_container"])
        .status()
        .expect("Failed to remove test container");
    println!("🛠 status: {:?}", status);
}
