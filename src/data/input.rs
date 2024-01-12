use crate::{
    aoc::fetch_input,
    clap::ARGS,
    data::INPUTS_DIR,
    years::{DAY_EXAMPLE, DAY_EXAMPLE_ANSWER},
};

use lazy_static::lazy_static;
use std::{fs, path::PathBuf};

lazy_static! {
    pub static ref YEAR_DIR: PathBuf = INPUTS_DIR.join(ARGS.lock().unwrap().year.to_string());
    pub static ref DAY_DIR: PathBuf = YEAR_DIR.join(ARGS.lock().unwrap().day.to_string());
    pub static ref INPUT_FILE: PathBuf = DAY_DIR.join(ARGS.lock().unwrap().input.clone());
    pub static ref ANSWER_FILE: PathBuf = DAY_DIR.join("answer.txt");
    pub static ref INPUT: String = {
        let mutex = ARGS.lock().unwrap();
        let args = mutex.clone();
        drop(mutex);
        if args.example {
            if let Some(example) = *DAY_EXAMPLE {
                return String::from(example);
            } else {
                panic!("No example input found for this part!");
            }
        }
        if !DAY_DIR.exists() {
            fs::create_dir_all(DAY_DIR.as_path()).expect("Failed to create day dir!")
        }
        if !INPUT_FILE.exists() {
            println!(
                "Year {} day {} input file not found! Downloading now!",
                args.year, args.day
            );
            drop(args);
            // print full path
            println!("{}", INPUT_FILE.as_path().display());
            return fetch_input();
        }

        return match fs::read_to_string(INPUT_FILE.as_path()) {
            Ok(input) => input,
            Err(err) => panic!("Error reading file! {}", err),
        };
    };
    pub static ref ANSWER: Option<String> = {
        let mutex = ARGS.lock().unwrap();
        let args = mutex.clone();
        drop(mutex);
        if args.example {
            if let Some(answer) = *DAY_EXAMPLE_ANSWER {
                return Some(String::from(answer));
            }
        }
        if !ANSWER_FILE.exists() {
            return None;
        }
        return match fs::read_to_string(ANSWER_FILE.as_path()) {
            Ok(answer) => Some(answer),
            Err(err) => panic!("Error reading file! {}", err),
        };
    };
}

impl std::fmt::Display for INPUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}
