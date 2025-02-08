use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(version, about = "A simple Docker container stopping tool")]
pub struct Args {
    #[arg(short = 'k', long = "kill")]
    pub kill: bool,

    #[arg(short = 'l', long = "ls", action = ArgAction::SetTrue)]
    pub ls: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
