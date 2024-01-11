use crate::clap::Args;

pub mod _2023;
pub fn run_year(args: &Args, input: &str) -> String {
    return match args.year {
        2023 => _2023::run_day(args, input),
        _ => panic!("Invalid Year!"),
    };
}
