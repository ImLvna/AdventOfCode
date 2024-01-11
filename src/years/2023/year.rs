use crate::years::config::Year;

#[path = "0/main.rs"]
pub mod _0;
#[path = "1/main.rs"]
pub mod _1;
#[path = "2/main.rs"]
pub mod _2;
#[path = "3/main.rs"]
pub mod _3;
#[path = "4/main.rs"]
pub mod _4;

lazy_static::lazy_static! {
    pub static ref YEAR: Year = Year {
        year: 2023,
        days: vec![
            *_0::DAY,
            *_1::DAY,
            *_2::DAY,
            *_3::DAY,
            *_4::DAY,
        ]
    };
}
