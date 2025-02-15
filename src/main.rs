mod cli;
mod dosukoi;
mod list;
mod logo;

fn main() {
    let args = cli::parse_args();
    logo::print_logo();

    if args.ls {
        let containers = list::get_running_containers();
        let selected = list::select_containers(&containers, args.kill);
        dosukoi::stop_or_kill_containers(&selected, args.kill);
    } else if let Some(project) = args.project {
        let containers = dosukoi::get_containers_by_project(&project);
        if containers.is_empty() {
            println!("No running containers found for project: {}", project);
        } else {
            dosukoi::stop_or_kill_containers(&containers, args.kill);
        }
    } else if let Some(containers) = dosukoi::get_running_containers() {
        dosukoi::stop_or_kill_containers(&containers, args.kill);
    } else {
        println!("No running containers found.");
    }
}
