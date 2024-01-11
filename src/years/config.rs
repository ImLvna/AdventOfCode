use crate::clap::ARGS;

#[derive(Clone)]
pub struct Year {
    pub year: u32,
    pub days: Vec<Day>,
}

impl Year {
    pub fn get_day(&self) -> Option<&Day> {
        return self.days.iter().find(|day| day.day == ARGS.day);
    }
}

#[derive(Clone, Copy)]
pub struct Day {
    pub day: u8,
    pub p1: fn() -> String,
    pub p2: fn() -> String,
}

impl Day {
    pub fn get_part(&self) -> fn() -> String {
        return match ARGS.part {
            1 => self.p1,
            2 => self.p2,
            _ => panic!("Unknown part"),
        };
    }
}
