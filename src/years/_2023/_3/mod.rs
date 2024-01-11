use std::vec;

use lazy_static::lazy_static;
use regex::Regex;

use crate::clap::Args;

lazy_static! {
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[^\w\d.]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d").unwrap();
}

fn parse_lines(lines: Vec<&str>) -> u32 {
    let mut top = lines.get(0).unwrap().chars();
    let mut cur = lines.get(1).unwrap().chars();
    let mut bottom = lines.get(2).unwrap().chars();
    let mut curtop = top.next().unwrap();
    let mut curmiddle = cur.next().unwrap();
    let mut curbottom = bottom.next().unwrap();

    let mut totalnumber = 0;

    let mut curnumber = 0;
    let mut cur_is_part = false;
    for ((nexttop, nextmiddle), nextbottom) in top.zip(cur).zip(bottom) {
        let _box = vec![
            [curtop, curmiddle, curbottom],
            [nexttop, nextmiddle, nextbottom],
        ];

        for line in _box.iter() {
            for _char in line.iter() {
                if SYMBOL_REGEX.is_match(_char.to_string().as_str()) {
                    cur_is_part = true;
                }
                if cur_is_part {
                    break;
                }
            }
            if cur_is_part {
                break;
            }
        }

        let next_is_number = NUMBER_REGEX.is_match(nextmiddle.to_string().as_str());

        if NUMBER_REGEX.is_match(curmiddle.to_string().as_str()) {
            curnumber = (curnumber * 10) + curmiddle.to_string().as_str().parse::<u32>().unwrap();
            if !next_is_number {
                if cur_is_part {
                    totalnumber += curnumber;

                    cur_is_part = false;
                }
                curnumber = 0;
            }
        } else {
            curnumber = 0;
            if !next_is_number {
                cur_is_part = false;
            }
        }
        curtop = nexttop;
        curmiddle = nextmiddle;
        curbottom = nextbottom;
    }
    return totalnumber;
}

fn pad(contents: String) -> String {
    let dots = ".".repeat(contents.lines().last().unwrap().len());
    // Add starting and ending dot lines
    return format!("{}\n{}\n{}", dots, contents, dots)
        // Add a dot to the start and end of every line
        .lines()
        .map(|line| ".".to_owned() + line + ".")
        .intersperse("\n".to_string())
        .collect();
}

pub fn main(args: &Args, input: &str) -> String {
    return match args.part {
        1 => p1(args, input),
        2 => p2(args, input),
        _ => panic!("Unknown part"),
    };
}

fn p1(_args: &Args, input: &str) -> String {
    let contents = pad(input.to_owned());

    let mut lines = contents.lines();

    let mut num = 0;

    let mut prev = lines.next().unwrap();
    let mut current = lines.next().unwrap();

    for next in lines {
        num += parse_lines(vec![prev, current, next]);
        prev = current;
        current = next;
    }

    // println!("{}", contents);
    return num.to_string();
}

pub fn p2(_args: &Args, input: &str) -> String {
    println!("{}", input);
    return 0.to_string();
}
