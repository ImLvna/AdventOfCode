use crate::clap::Args;

fn get_input(args: &Args) -> &'static str {
    match args.input {
        false => include_str!("../../../inputs/1/example.txt"),
        true => include_str!("../../../inputs/1/input.txt"),
    }
}

pub fn main(args: &Args) -> String {
    return match args.part {
        1 => p1(args),
        2 => p2(args),
        _ => panic!("Unknown part"),
    };
}

pub fn p1(args: &Args) -> String {
    let contents = get_input(&args);
    println!("{}", contents);
    return 0.to_string();
}

pub fn p2(args: &Args) -> String {
    let contents = get_input(&args);
    println!("{}", contents);
    return 0.to_string();
}
