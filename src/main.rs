use clap::Parser;
use std::process::Command;

/// DOSUKOI: A simple Docker container stopping tool
#[derive(Parser)]
#[command(version, about = "A simple Docker container stopping tool")]
#[clap(trailing_var_arg = true)] // ← これを追加！
struct Args {
    /// Kill containers instead of stopping them
    #[arg(short = 'k', long = "kill")]
    kill: bool,

    /// Specify a Docker Compose project to stop
    #[arg(trailing_var_arg = true, num_args = 0..)]
    project: Option<String>,
}

fn main() {
    let args = Args::parse();

    // `docker stop` or `docker kill`
    let command = if args.kill { "kill" } else { "stop" };

    // ロゴの表示
    println!(
        r#"
██████╗  ██████╗ ███████╗██╗   ██╗██╗  ██╗ ██████╗  ██████╗ 
██╔══██╗██╔═══██╗██╔════╝██║   ██║██║ ██╔╝██╔═══██╗  ╚██╔╝  
██║  ██║██║   ██║███████╗██║   ██║█████╔╝ ██║   ██║   ██║  
██║  ██║██║   ██║╚════██║██║   ██║██╔═██╗ ██║   ██║   ██║  
██████╔╝╚██████╔╝███████║╚██████╔╝██║  ██╗╚███ ███╔╝██████║  
╚═════╝  ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝  ╚═╝ ╚═══╝╚═╝   ╚═╝  
"#
    );

    if let Some(project) = args.project {
        println!("Stopping Docker Compose project: {}", project);
        let output = Command::new("docker")
            .args(["compose", "-p", &project, command])
            .output()
            .expect("Failed to execute docker compose");

        if output.status.success() {
            println!("(╯°□°）╯︵ ┻━┻\n{} containers {}ed!", project, command);
        } else {
            eprintln!(
                "Error stopping {}: {}",
                project,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else {
        println!("Stopping all running Docker containers...");
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("docker {} $(docker ps -q)", command))
            .output()
            .expect("Failed to execute docker command");

        if output.status.success() {
            println!("(╯°□°）╯︵ ┻━┻\nAll containers {}ed!", command);
        } else {
            eprintln!(
                "Error stopping containers: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
