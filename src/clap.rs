use clap::{arg, command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "example.txt")]
    pub input: String,

    #[arg(short, long)]
    pub stopwatch: bool,

    pub year: u32,
    pub day: u8,
    pub part: u8,
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
