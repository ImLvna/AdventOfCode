use crate::clap::Args;

pub fn main(args: &Args, input: &str) -> String {
    return match args.part {
        1 => p1(args, input),
        2 => p2(args, input),
        _ => panic!("Unknown part"),
    };
}

pub fn p1(_args: &Args, input: &str) -> String {
    println!("{}", input);
    return 0.to_string();
}

pub fn p2(_args: &Args, input: &str) -> String {
    println!("{}", input);
    return 0.to_string();
}
