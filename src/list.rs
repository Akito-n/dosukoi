use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::process::Command;
use std::str;

pub fn get_running_containers() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("docker ps --format '{{.Names}}'")
        .output()
        .expect("Failed to execute docker ps");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        stdout.lines().map(|s| s.to_string()).collect()
    } else {
        vec![]
    }
}

pub fn select_containers(containers: &[String], kill: bool) -> Vec<String> {
    if containers.is_empty() {
        println!("No running containers found.");
        return vec![];
    }

    let action = if kill { "kill" } else { "stop" };

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("Select containers to {} >", action))
        .items(containers)
        .interact()
        .unwrap();

    selections.iter().map(|&i| containers[i].clone()).collect()
}
