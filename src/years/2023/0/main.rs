use crate::data::input::INPUT;
#[cfg(test)]
mod tests;

pub fn p1() -> String {
    println!("{}", INPUT);
    return 1.to_string();
}

pub fn p2() -> String {
    println!("{}", INPUT);
    return 2.to_string();
}

// Day Meta
lazy_static::lazy_static! {
    pub static ref DAY: crate::types::Day = crate::types::Day {
        day: 0,
        parts: [
            &crate::types::Part {
                func: p1,
                example: Some(include_str!("example1.txt")),
                example_answer: Some(include_str!("example1_answer.txt")),
            },
            &crate::types::Part {
                func: p2,
                example: Some(include_str!("example2.txt")),
                example_answer: Some(include_str!("example2_answer.txt")),
            },
        ]

    };
}
