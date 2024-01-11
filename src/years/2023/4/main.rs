use crate::data::input::INPUT;

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
        day:4,
        p1,
        p2,

    };
}
