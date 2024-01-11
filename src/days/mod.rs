use crate::{clap::Args, days::_0::template};

pub mod _0;

pub fn run_day(args: &Args) -> String {
    return match args.day {
        0 => template(args), // Day 0 is a template
        _ => panic!("Invalid Day!"),
    };
}
