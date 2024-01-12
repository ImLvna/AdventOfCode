pub mod years;

use lazy_static::lazy_static;

use crate::{
    clap::ARGS,
    types::{Day, Part, Year},
};

lazy_static! {
    pub static ref YEAR: &'static Year = {
        let args = ARGS.lock().unwrap();
        return years::YEARS
            .iter()
            .find(|year| year.year == args.year)
            .expect(&format!("Unknown Year {}!", args.year)[..]);
    };
    pub static ref DAY: &'static Day = YEAR.get_day().expect("Unknown Day");
    pub static ref PART: &'static Part = DAY.get_part();
    pub static ref DAY_FUNC: fn() -> String = PART.func;
    pub static ref DAY_EXAMPLE: Option<&'static str> = PART.example;
    pub static ref DAY_EXAMPLE_ANSWER: Option<&'static str> = PART.example_answer;
}
