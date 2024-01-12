use crate::types::PartEnum;
use clap::{arg, command, Parser};
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "input.txt")]
    pub input: String,

    #[arg(short, long)]
    pub example: bool,

    #[arg(short, long)]
    pub stopwatch: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    pub year: u32,
    pub day: u8,
    #[arg(value_enum)]
    pub part: PartEnum,
}

impl Args {
    pub fn new() -> Self {
        return Args {
            input: "input.txt".to_owned(),
            example: false,
            stopwatch: false,
            verbose: 0,
            year: 0,
            day: 0,
            part: PartEnum::PartOne,
        };
    }
}

use lazy_static::lazy_static;

use std::sync::Mutex;
lazy_static! {
    pub static ref ARGS: Mutex<Args> = Mutex::new(match cfg!(test) {
        true => Args::new(),
        false => Args::parse(),
    });
}

#[cfg(test)]
pub fn override_args(args: Args) {
    use std::ops::Deref;
    let mut mut_args = ARGS.lock().unwrap();
    let _ = mut_args.deref();
    *mut_args = args;
    drop(mut_args);
}

#[cfg(test)]
pub fn reset_args() {
    override_args(Args::new());
}
