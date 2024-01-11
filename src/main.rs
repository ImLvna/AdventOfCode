use clap::get_args;

use crate::years::run_year;

pub mod clap;
pub mod years;
fn main() {
    let args = get_args();
    println!(
        "Running {} day {} part {} with {} input",
        args.year,
        args.day,
        args.part,
        match args.input {
            false => "example",
            true => "real",
        }
    );
    let output = run_year(&args);
    println!("Output: {}", output);
}
