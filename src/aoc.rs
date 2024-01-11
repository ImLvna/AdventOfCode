use std::{fs, process};

use crate::{
    clap::ARGS,
    data::{input::INPUT_FILE, COOKIE_FILE},
};
use aoc_client::AocClient;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLIENT: AocClient = {
        if !COOKIE_FILE.exists() {
            request_session_cookie();
        }
        return AocClient::builder()
            .session_cookie_from_file(COOKIE_FILE.as_path())
            .expect("Failed to read cookie file!")
            .year(ARGS.year.try_into().unwrap())
            .expect("Failed to set client year!")
            .day(ARGS.day.try_into().unwrap())
            .expect("Failed to set client day!")
            .build()
            .expect("Failed to build aoc client!");
    };
}

fn request_session_cookie() {
    let prompt = inquire::Password::new("Please enter your session cookie")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt();

    match prompt {
        Ok(cookie) => fs::write(COOKIE_FILE.as_path(), cookie).expect("Failed to write file!"),
        Err(_) => process::exit(1),
    }
}

pub fn fetch_input() -> String {
    let input = CLIENT.get_input().expect("Failed to get input!");
    fs::write(INPUT_FILE.as_path(), &input).expect("Failed to write input file!");
    return input;
}
