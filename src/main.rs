#![feature(iter_intersperse)]
#![feature(fs_try_exists)]

use crate::{clap::ARGS, data::check_data_dir, years::get_year};

pub mod aoc;
pub mod clap;
pub mod data;
pub mod years;
fn main() {
    let program_stopwatch = std::time::Instant::now();

    check_data_dir();

    let year = get_year();
    let day = year.get_day().expect("Unknown day");
    let part = day.get_part();

    println!(
        "Running {} day {} part {} with {} input",
        ARGS.year, ARGS.day, ARGS.part, ARGS.input
    );
    let stopwatch = std::time::Instant::now();
    let output = part();
    println!("Output: {}", output);

    if ARGS.stopwatch {
        println!("Finished in {}ms", program_stopwatch.elapsed().as_millis());
        println!("Day took {}ms", stopwatch.elapsed().as_millis());
    }
}
