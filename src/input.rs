use crate::clap::Args;

use std::{fs, path::PathBuf, str::FromStr};

pub fn get_input(args: &Args) -> String {
    let mut path = PathBuf::from_str("./inputs").unwrap();
    println!("{}", fs::canonicalize(&path).unwrap().to_str().unwrap());
    match path.try_exists() {
        Err(_) => panic!("Inputs folder not found!"),
        Ok(_) => {}
    };
    path = path.join(args.year.to_string());
    match path.try_exists() {
        Err(_) => panic!("Year {} inputs folder not found!", args.year),
        Ok(_) => {}
    };
    path = path.join(args.day.to_string());
    match path.try_exists() {
        Err(_) => panic!(
            "Year {} day {} inputs folder not found!",
            args.year, args.day
        ),
        Ok(_) => {}
    };
    path = path.join(args.input.to_string());
    match path.try_exists() {
        Err(_) => panic!("Input file {} not found!", path.to_str().unwrap()),
        Ok(_) => {}
    };

    return match fs::read_to_string(path) {
        Ok(input) => input,
        Err(err) => panic!("Error reading file! {}", err),
    };
}
