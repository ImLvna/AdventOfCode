use crate::{clap::ARGS, data::input::INPUT};

pub fn main() -> String {
    return match ARGS.part {
        1 => p1(),
        2 => p2(),
        _ => panic!("Unknown part"),
    };
}

pub fn p1() -> String {
    println!("{}", INPUT);
    return 0.to_string();
}

pub fn p2() -> String {
    println!("{}", INPUT);
    return 0.to_string();
}

lazy_static::lazy_static! {
    pub static ref DAY: crate::years::config::Day = crate::years::config::Day {
        day: 0,
        p1,
        p2,

    };
}
