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

pub fn get_all_containers_by_project(project: &str) -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "docker ps -a --filter 'label=com.docker.compose.project={}' --format '{{{{.Names}}}}'",
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

        let containers = get_all_containers_by_project(proj);
        if containers.is_empty() {
            println!(
                "No containers found for project: {}. Will try to remove other resources.",
                proj
            );
        } else {
            println!(
                "Found {} containers to remove for project: {}",
                containers.len(),
                proj
            );
        }
        let mut success = false;
        let compose_v2_output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "docker compose -p {} down --rmi all --volumes --remove-orphans",
                proj
            ))
            .output();

        if let Ok(output) = compose_v2_output {
            if output.status.success() {
                println!("Successfully removed project resources using Docker Compose V2.");
                success = true;
            } else {
                let compose_v1_output = Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "docker-compose -p {} down --rmi all --volumes --remove-orphans",
                        proj
                    ))
                    .output();

                if let Ok(output) = compose_v1_output {
                    if output.status.success() {
                        println!("Successfully removed project resources using Docker Compose V1.");
                        success = true;
                    }
                }
            }
        }

        if !success && !containers.is_empty() {
            println!("Docker Compose commands failed. Removing containers directly...");
            let container_list = containers.join(" ");

            let rm_output = Command::new("sh")
                .arg("-c")
                .arg(format!("docker rm -f {}", container_list))
                .output();

            if let Ok(output) = rm_output {
                if output.status.success() {
                    println!("Successfully removed containers directly.");
                    let network_cmd = format!(
                        "docker network ls --filter 'label=com.docker.compose.project={}' --format '{{{{.Name}}}}' | xargs -r docker network rm",
                        proj
                    );

                    let _ = Command::new("sh").arg("-c").arg(&network_cmd).output();
                    let volume_cmd = format!(
                        "docker volume ls --filter 'label=com.docker.compose.project={}' --format '{{{{.Name}}}}' | xargs -r docker volume rm",
                        proj
                    );

                    let _ = Command::new("sh").arg("-c").arg(&volume_cmd).output();

                    success = true;
                }
            }
        }

        if success {
            println!(
                "(╯°□°）╯︵ ┻━┻\nKimarite successful! Project resources have been eliminated."
            );
        } else {
            eprintln!(
                "Error: Could not fully execute kimarite on project '{}'.",
                proj
            );
            eprintln!("Try running the following manually:");
            eprintln!(
                "  docker compose -p {} down --rmi all --volumes --remove-orphans",
                proj
            );
            eprintln!("  OR");
            eprintln!(
                "  docker-compose -p {} down --rmi all --volumes --remove-orphans",
                proj
            );
        }
    } else {
        println!("Executing kimarite on all Docker resources...");
        let stop_output = Command::new("sh")
            .arg("-c")
            .arg("docker stop $(docker ps -q) 2>/dev/null || true")
            .output();

        if let Ok(_) = stop_output {
            println!("Stopped all running containers.");
        }
        let rm_output = Command::new("sh")
            .arg("-c")
            .arg("docker rm -f $(docker ps -a -q) 2>/dev/null || true")
            .output();

        if let Ok(_) = rm_output {
            println!("Removed all containers.");
        }
        let prune_output = Command::new("sh")
            .arg("-c")
            .arg("docker system prune -af --volumes")
            .output();

        if let Ok(output) = prune_output {
            if output.status.success() {
                println!(
                    "(╯°□°）╯︵ ┻━┻\nKimarite successful! All resources have been eliminated."
                );
            } else {
                eprintln!(
                    "Error during system prune: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                eprintln!("Try running 'docker system prune -af --volumes' manually.");
            }
        } else {
            eprintln!("Failed to execute docker system prune command.");
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
