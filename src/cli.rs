use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(version, about = "A simple Docker container stopping tool")]
pub struct Args {
    #[arg(long = "kimarite", short = 'K', action = ArgAction::SetTrue)]
    pub kimarite: bool,

    #[arg(short = 'k', long = "kill")]
    pub kill: bool,

    #[arg(short = 'l', long = "ls", action = ArgAction::SetTrue)]
    pub ls: bool,

    #[arg(help = "Specify Docker Compose project name", required = false)]
    pub project: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
