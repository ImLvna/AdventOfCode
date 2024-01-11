use clap::{arg, command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: bool,

    pub year: u32,
    pub day: u8,
    pub part: u8,
}

pub fn get_args() -> Args {
    return Args::parse();
}
