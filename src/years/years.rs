use lazy_static::lazy_static;

use crate::types::Year;

#[path = "2023/year.rs"]
pub mod _2023;

lazy_static! {
    pub static ref YEARS: Vec<Year> = vec![_2023::YEAR.clone()];
}
