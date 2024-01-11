use crate::{clap::Args, days::_0::template};

use self::_1::day1;

pub mod _0;
pub mod _1;

pub fn run_day(args: &Args) -> String {
    return match args.day {
        0 => template(args), // Day 0 is a template
        1 => day1(args),
        _ => panic!("Invalid Day!"),
    };
}
