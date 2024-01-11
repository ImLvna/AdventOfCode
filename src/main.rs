use clap::get_args;
use days::run_day;

pub mod clap;
pub mod days;
fn main() {
    let args = get_args();
    println!("Running day {} with {} input", args.day, args.input);
    let output = run_day(&args);
    println!("Output: {}", output);
}
