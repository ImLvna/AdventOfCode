use crate::clap::{Args, InputType};

fn get_input(args: &Args) -> &'static str {
    match args.input {
        InputType::Example => include_str!("../../../inputs/1/example.txt"),
        InputType::Final => include_str!("../../../inputs/1/input.txt"),
    }
}

pub fn template(args: &Args) -> String {
    let contents = get_input(&args);
    println!("{}", contents);
    return 0.to_string();
}
