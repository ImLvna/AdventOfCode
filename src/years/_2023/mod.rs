use crate::clap::Args;

pub mod _0;
pub mod _1;
pub mod _2;
pub mod _3;

pub fn run_day(args: &Args, input: &str) -> String {
    return match args.day {
        0 => _0::main(args, input), // Day 0 is a template
        1 => _1::main(args, input),
        2 => _2::main(args, input),
        3 => _3::main(args, input),
        _ => panic!("Invalid Day!"),
    };
}
