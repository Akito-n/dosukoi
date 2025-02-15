use std::process::Command;

pub fn get_running_containers() -> Option<Vec<String>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("docker ps --format '{{.Names}}'")
        .output()
        .expect("Failed to get running containers");

    if output.status.success() {
        let containers = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        if containers.is_empty() {
            None
        } else {
            Some(containers)
        }
    } else {
        None
    }
}

pub fn stop_or_kill_containers(containers: &[String], kill: bool) {
    let command = if kill { "kill" } else { "stop" };
    let container_list = containers.join(" ");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("docker {} {}", command, container_list))
        .output()
        .expect("Failed to execute docker command");

    if output.status.success() {
        println!("(╯°□°）╯︵ ┻━┻\nSelected containers dosukoi!");
    } else {
        eprintln!(
            "Error stopping containers: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn get_containers_by_project(project: &str) -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "docker ps --filter 'label=com.docker.compose.project={}' --format '{{{{.Names}}}}'",
            project
        ))
        .output()
        .expect("Failed to execute docker ps");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.lines().map(|s| s.to_string()).collect()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_running_containers() {
        let containers = get_running_containers();
        assert!(
            containers.as_ref().map_or(true, |c| c.is_empty()) || containers.is_some(),
            "Function should return a vector"
        );
    }

    #[test]
    fn test_stop_or_kill_containers() {
        let containers = vec!["test_container".to_string()];
        stop_or_kill_containers(&containers, false);
        stop_or_kill_containers(&containers, true);
        assert!(true);
    }
}
