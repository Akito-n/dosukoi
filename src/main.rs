use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]
struct Args {
    project: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(project) = args.project {
        println!("Stopping Docker Compose project: {}", project);
        let output = Command::new("docker")
            .args(["compose", "-p", &project, "stop"])
            .output()
            .expect("Failed to execute docker compose stop");

        if output.status.success() {
            println!("(╯°□°）╯︵ ┻━┻\n{} containers stopped!", project);
        } else {
            eprintln!("Error stopping {}: {}", project, String::from_utf8_lossy(&output.stderr));
        }
    } else {
        println!("Stopping all running Docker containers...");
        let output = Command::new("sh")
            .arg("-c")
            .arg("docker stop $(docker ps -q)")
            .output()
            .expect("Failed to execute docker stop");

        if output.status.success() {
            println!("(╯°□°）╯︵ ┻━┻\nAll containers stopped!");
        } else {
            eprintln!("Error stopping containers: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
