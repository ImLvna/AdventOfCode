use crate::clap::Args;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HANDFUL_REGEX: Regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
    static ref GAME_ID_REGEX: Regex = Regex::new(r"Game (\d+)").unwrap();
}

trait IsValid {
    fn is_valid(&self) -> bool;
}
trait GetMax {
    fn get_maximums(&self) -> Round;
}

#[derive(Copy, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
impl IsValid for Round {
    fn is_valid(&self) -> bool {
        if self.red > 12 {
            return false;
        };
        if self.green > 13 {
            return false;
        };
        if self.blue > 14 {
            return false;
        };
        return true;
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}
impl IsValid for Game {
    fn is_valid(&self) -> bool {
        for round in self.rounds.iter() {
            if !round.is_valid() {
                return false;
            }
        }
        return true;
    }
}
impl GetMax for Game {
    fn get_maximums(&self) -> Round {
        let mut maxs = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        for round in self.rounds.iter() {
            if round.red > maxs.red {
                maxs.red = round.red
            };
            if round.green > maxs.green {
                maxs.green = round.green
            };
            if round.blue > maxs.blue {
                maxs.blue = round.blue
            };
        }
        return maxs;
    }
}

fn parse_round(line: &str) -> Round {
    // println!("{}", line);
    let handfuls = line
        .split(", ")
        .map(|handful| HANDFUL_REGEX.captures(handful).unwrap());
    // Group 1: Number
    // Group 2: Color
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for handful in handfuls {
        let num = handful.get(1).unwrap().as_str().parse::<u32>().unwrap();
        match handful.get(2).unwrap().as_str() {
            "red" => round.red = num,
            "green" => round.green = num,
            "blue" => round.blue = num,
            _ => {}
        }
    }
    return round;
}

fn parse_line(line: &str) -> Game {
    let id: u32 = GAME_ID_REGEX
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    // println!("{}", id);
    let rounds = line
        .split("; ")
        .map(|round| parse_round(round))
        .collect::<Vec<Round>>();
    return Game {
        id: id,
        rounds: rounds,
    };
}
pub fn main(args: &Args, input: &str) -> String {
    return match args.part {
        1 => p1(args, input),
        2 => p2(args, input),
        _ => panic!("Unknown part"),
    };
}

fn p1(_args: &Args, input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_line(line);
        if game.is_valid() {
            sum += game.id
        };
    }
    return sum.to_string();
}
pub fn p2(_args: &Args, input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_line(line);
        let maximums = game.get_maximums();
        let power = maximums.red * maximums.green * maximums.blue;
        sum += power;
    }
    return sum.to_string();
}
