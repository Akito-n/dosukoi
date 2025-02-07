use clap::Parser;

#[derive(Parser)]
#[command(version, about = "A simple Docker container stopping tool")]
pub struct Args {
    #[arg(short = 'k', long = "kill")]
    pub kill: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
