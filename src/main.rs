#![feature(iter_intersperse)]
#![feature(fs_try_exists)]

use clap::get_args;

use crate::{input::get_input, years::run_year};

pub mod clap;
pub mod input;
pub mod years;
fn main() {
    let args = get_args();

    let input = get_input(&args);

    println!(
        "Running {} day {} part {} with {} input",
        args.year, args.day, args.part, args.input
    );
    let stopwatch = std::time::Instant::now();
    let output = run_year(&args, &input);
    println!("Output: {}", output);

    if args.stopwatch {
        println!("Finished in {}ms", stopwatch.elapsed().as_millis())
    }
}
