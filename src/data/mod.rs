use std::{fs, path::PathBuf, str::FromStr};

use lazy_static::lazy_static;
pub mod input;

lazy_static! {
    pub static ref DATA_DIR: PathBuf = PathBuf::from_str("./data").unwrap();
    pub static ref INPUTS_DIR: PathBuf = DATA_DIR.join("inputs");
    pub static ref COOKIE_FILE: PathBuf = DATA_DIR.join(".sessioncookie");
}

pub fn check_data_dir() {
    if !match DATA_DIR.try_exists() {
        Ok(exists) => exists,
        Err(_) => false,
    } {
        println!("Making data dir...");
        fs::create_dir(DATA_DIR.as_path()).expect("Unable to create data dir!");
    }
}
