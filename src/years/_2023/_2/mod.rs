use crate::clap::Args;
use regex::Regex;

fn get_input(args: &Args) -> &'static str {
    match args.input {
        false => include_str!("../../../../inputs/2/example.txt"),
        true => include_str!("../../../../inputs/2/input.txt"),
    }
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
    let handful_regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();
    let handfuls = line
        .split(", ")
        .map(|handful| handful_regex.captures(handful).unwrap());
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
    let game_id_regex = Regex::new(r"Game (\d+)").unwrap();
    let id: u32 = game_id_regex
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
pub fn main(args: &Args) -> String {
    return match args.part {
        1 => p1(args),
        2 => p2(args),
        _ => panic!("Unknown part"),
    };
}

pub fn p1(args: &Args) -> String {
    let contents = get_input(&args);
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_line(line);
        if game.is_valid() {
            sum += game.id
        };
    }
    return sum.to_string();
}
pub fn p2(args: &Args) -> String {
    let contents = get_input(&args);
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        let game = parse_line(line);
        let maximums = game.get_maximums();
        let power = maximums.red * maximums.green * maximums.blue;
        sum += power;
    }
    return sum.to_string();
}