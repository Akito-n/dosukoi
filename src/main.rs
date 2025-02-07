mod cli;
mod dosukoi;
mod logo;

fn main() {
    let args = cli::parse_args();
    logo::print_logo();

    if let Some(containers) = dosukoi::get_running_containers() {
        dosukoi::stop_or_kill_containers(&containers, args.kill);
    } else {
        println!("No running containers found.");
    }
}
