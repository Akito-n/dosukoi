mod cli;
mod dosukoi;
mod logo;

fn main() {
    let args = cli::parse_args(); // CLI引数を解析
    logo::print_logo(); // ロゴ表示

    if let Some(containers) = dosukoi::get_running_containers() {
        dosukoi::stop_or_kill_containers(&containers, args.kill);
    } else {
        println!("No running containers found.");
    }
}
