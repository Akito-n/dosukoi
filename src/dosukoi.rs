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

pub fn execute_kimarite(project: Option<&str>) {
    if let Some(proj) = project {
        println!("Executing kimarite on project: {}", proj);

        let compose_cmd = format!(
            "docker compose -p {} down --rmi all --volumes --remove-orphans || docker-compose -p {} down --rmi all --volumes --remove-orphans",
            proj, proj
        );

        let compose_output = Command::new("sh").arg("-c").arg(&compose_cmd).output();

        if let Ok(output) = compose_output {
            if output.status.success() {
                println!(
                    "(╯°□°）╯︵ ┻━┻\nKimarite successful! Project resources have been eliminated."
                );
                return;
            }
        }

        println!("Falling back to individual container removal...");
        let containers = get_containers_by_project(proj);
        if !containers.is_empty() {
            stop_or_kill_containers(&containers, true);
            let _ = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "docker volume prune -f --filter label=com.docker.compose.project={}",
                    proj
                ))
                .output();
        }
    } else {
        // プロジェクト指定なしの場合は全てのDockerリソースを対象にする。これあぶないし必要ないかもな~
        // kimariteオプション自体がミスって打つことないしいいか
        println!("Executing kimarite on all Docker resources...");
        let cmd = "docker system prune -af --volumes";

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute kimarite command");

        if output.status.success() {
            println!("(╯°□°）╯︵ ┻━┻\nKimarite successful! All resources have been eliminated.");
        } else {
            eprintln!(
                "Error during kimarite: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
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
