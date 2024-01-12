use crate::data::input::INPUT;

pub fn p1() -> String {
    println!("{}", INPUT);
    return 0.to_string();
}

pub fn p2() -> String {
    println!("{}", INPUT);
    return 0.to_string();
}

// Day Meta
lazy_static::lazy_static! {
    pub static ref DAY: crate::types::Day = crate::types::Day {
        day: 4,
        parts: [
            &crate::types::Part {
                func: p1,
                example: Some(include_str!("example.txt")),
                example_answer: Some(include_str!("example_answer.txt")),
            },
            &crate::types::Part {
                func: p2,
                example: Some(include_str!("example.txt")),
                example_answer: Some(include_str!("example_answer.txt")),
            },
        ]

    };
}
