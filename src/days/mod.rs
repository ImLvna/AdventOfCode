use crate::clap::Args;

pub mod _0;
pub mod _1;
pub mod _2;

pub fn run_day(args: &Args) -> String {
    return match args.day {
        0 => _0::main(args), // Day 0 is a template
        1 => _1::main(args),
        2 => _2::main(args),
        _ => panic!("Invalid Day!"),
    };
}
