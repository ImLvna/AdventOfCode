use std::{collections::HashMap, vec};

use lazy_static::lazy_static;
use regex::Regex;

use crate::clap::Args;

lazy_static! {
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[^\w\d.]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d").unwrap();
}

fn is_number(_char: char) -> bool {
    return NUMBER_REGEX.is_match(_char.to_string().as_str());
}
fn is_symbol(_char: char) -> bool {
    return SYMBOL_REGEX.is_match(_char.to_string().as_str());
}
fn is_asterisk(_char: char) -> bool {
    return _char.to_string().as_str() == "*";
}

// numbers, stars
fn parse_lines(lines: Vec<&str>) -> (u32, Vec<((u32, u32), u32)>) {
    // part 2
    let mut stars: Vec<((u32, u32), u32)> = vec![];

    let mut top = lines.get(0).unwrap().chars();
    let mut cur = lines.get(1).unwrap().chars();
    let mut bottom = lines.get(2).unwrap().chars();
    let mut curtop = top.next().unwrap();
    let mut curmiddle = cur.next().unwrap();
    let mut curbottom = bottom.next().unwrap();

    let mut totalnumber = 0;

    let mut curnumber = 0;
    let mut cur_is_part = false;
    let mut cur_is_star = false; // p2
    let mut cur_star_coords: (u32, u32) = (0, 0);
    for (i, ((nexttop, nextmiddle), nextbottom)) in top.zip(cur).zip(bottom).enumerate() {
        let _box = vec![
            [curtop, curmiddle, curbottom],
            [nexttop, nextmiddle, nextbottom],
        ];

        for (j, line) in _box.iter().enumerate() {
            for (a, _char) in line.iter().enumerate() {
                if is_symbol(*_char) {
                    cur_is_part = true;
                    if is_asterisk(*_char) {
                        cur_is_star = true;
                        cur_star_coords = (
                            (i + 1 + j).try_into().unwrap(), // next index - 1 + (0 or 1, cur / next)
                            a.try_into().unwrap(),           // y
                        )
                    }
                }
                if cur_is_part {
                    break;
                }
            }
            if cur_is_part {
                break;
            }
        }

        let next_is_number = is_number(nextmiddle);

        if is_number(curmiddle) {
            curnumber = (curnumber * 10) + curmiddle.to_string().as_str().parse::<u32>().unwrap();
            if !next_is_number {
                if cur_is_part {
                    totalnumber += curnumber;

                    cur_is_part = false;

                    if cur_is_star {
                        stars.push((cur_star_coords, curnumber));
                        cur_is_star = false;
                        cur_star_coords = (0, 0);
                    }
                }
                curnumber = 0;
            }
        } else {
            curnumber = 0;
            if !next_is_number {
                cur_is_part = false;
                cur_is_star = false;
            }
        }
        curtop = nexttop;
        curmiddle = nextmiddle;
        curbottom = nextbottom;
    }
    return (totalnumber, stars);
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
        num += parse_lines(vec![prev, current, next]).0;
        prev = current;
        current = next;
    }

    // println!("{}", contents);
    return num.to_string();
}

pub fn p2(_args: &Args, input: &str) -> String {
    let contents = pad(input.to_owned());

    let mut lines = contents.lines();

    let mut stars: Vec<((u32, u32), u32)> = vec![];

    let mut prev = lines.next().unwrap();
    let mut current = lines.next().unwrap();

    for (i, next) in lines.enumerate() {
        let newstars = parse_lines(vec![prev, current, next]).1;
        let correctpositions: Vec<((u32, u32), u32)> = newstars
            .iter()
            .map(|star| ((star.0 .0, u32::try_from(i).unwrap() + star.0 .1), star.1))
            .collect();
        for correct in correctpositions {
            stars.push(correct);
        }
        prev = current;
        current = next;
    }

    // filter duplicated stars

    let duplicatedstars: Vec<&((u32, u32), u32)> = stars
        .iter()
        .filter(|star| stars.iter().filter(|newstar| newstar.0 == star.0).count() == 2)
        .collect();

    let mut gears: HashMap<(u32, u32), u32> = HashMap::new();
    for star in duplicatedstars {
        let val = gears.entry(star.0).or_insert(1);
        *val *= star.1;
    }

    println!("{:?}", gears);

    return gears.values().sum::<u32>().to_string();
}
