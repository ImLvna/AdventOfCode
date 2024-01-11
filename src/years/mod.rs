use crate::clap::ARGS;

use self::config::Year;

pub mod config;
pub mod years;

pub fn get_year() -> &'static Year {
    return years::YEARS
        .iter()
        .find(|year| year.year == ARGS.year)
        .unwrap();
}
