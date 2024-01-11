use crate::{aoc::fetch_input, clap::ARGS, data::INPUTS_DIR};

use lazy_static::lazy_static;
use std::{fs, path::PathBuf};

lazy_static! {
    pub static ref YEAR_DIR: PathBuf = INPUTS_DIR.join(ARGS.year.to_string());
    pub static ref DAY_DIR: PathBuf = YEAR_DIR.join(ARGS.day.to_string());
    pub static ref INPUT_FILE: PathBuf = DAY_DIR.join(ARGS.input.clone());
    pub static ref INPUT: String = {
        if !DAY_DIR.exists() {
            fs::create_dir_all(DAY_DIR.as_path()).expect("Failed to create day dir!")
        }
        if !INPUT_FILE.exists() {
            println!(
                "Year {} day {} input file not found! Downloading now!",
                ARGS.year, ARGS.day
            );
            return fetch_input();
        }

        return match fs::read_to_string(INPUT_FILE.as_path()) {
            Ok(input) => input,
            Err(err) => panic!("Error reading file! {}", err),
        };
    };
}

impl std::fmt::Display for INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}
