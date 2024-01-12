use std::str::FromStr;

use clap::ValueEnum;

use crate::clap::ARGS;

#[derive(Clone)]
pub struct Year {
    pub year: u32,
    pub days: Vec<Day>,
}

impl Year {
    pub fn get_day(&self) -> Option<&Day> {
        return self
            .days
            .iter()
            .find(|day| day.day == ARGS.lock().unwrap().day);
    }
}

#[derive(Clone, Copy)]
pub struct Day {
    pub day: u8,
    pub parts: [&'static Part; 2],
}

impl Day {
    pub fn get_part(&self) -> &Part {
        return self
            .parts
            .get(ARGS.lock().unwrap().part.to_u8() as usize - 1)
            .expect("Unknown Part");
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartEnum {
    PartOne,
    PartTwo,
}
impl ValueEnum for PartEnum {
    fn value_variants<'a>() -> &'a [Self] {
        &[PartEnum::PartOne, PartEnum::PartTwo]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_str()))
    }
}
impl PartEnum {
    pub fn to_str(&self) -> &'static str {
        match self {
            PartEnum::PartOne => "1",
            PartEnum::PartTwo => "2",
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            PartEnum::PartOne => 1,
            PartEnum::PartTwo => 2,
        }
    }
}
impl FromStr for PartEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(PartEnum::PartOne),
            "2" => Ok(PartEnum::PartTwo),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PartEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u8())
    }
}

pub struct Part {
    pub func: fn() -> String,
    pub example: Option<&'static str>,
    pub example_answer: Option<&'static str>,
}
