#![feature(iter_intersperse)]
#![feature(fs_try_exists)]

use crate::{
    clap::ARGS,
    data::{check_data_dir, input::ANSWER},
    stopwatch::Stopwatch,
    years::PART,
};

pub mod aoc;
pub mod clap;
pub mod data;
pub mod stopwatch;
#[cfg(test)]
pub mod tests;
pub mod types;
pub mod years;

fn main_shim() -> String {
    let mut phase_stopwatch = Stopwatch::new("Initalization".to_owned());

    check_data_dir();

    let part = *PART;

    let args = ARGS.lock().unwrap();

    println!(
        "Running {} day {} part {} with {} input",
        args.year,
        args.day,
        args.part,
        match args.example {
            false => args.input.clone(),
            true => String::from("example"),
        }
    );

    drop(args);

    phase_stopwatch
        .stop()
        .print()
        .set_name("Day".to_owned())
        .reset();
    let output = (part.func)();
    phase_stopwatch.stop();
    println!("Output: {}", output);
    if let Some(answer) = ANSWER.clone() {
        println!(
            "The answer is {}correct",
            if output == answer { "" } else { "in" }
        );
    }
    phase_stopwatch.print();
    return output;
}

fn main() {
    let mut root_stopwatch = Stopwatch::new("Program".to_owned());
    main_shim();
    root_stopwatch.stop().print();
}
