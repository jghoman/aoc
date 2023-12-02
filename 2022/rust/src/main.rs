use crate::Move::{Paper, Rock, Scissors};
use crate::Result::{IWin, TheyWin, Draw};

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beaten_by(&self) -> Move {
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn beats(&self) -> Move {
        match *self {
            Paper => Rock,
            Scissors => Paper,
            Rock => Scissors,
        }
    }
}

#[derive(Debug)]
enum Result {
    IWin,
    TheyWin,
    Draw,
}

#[derive(Debug, PartialEq)]
struct RockPaperScissorsGame {
    their_move: Move,
    my_move: Move,
}

impl RockPaperScissorsGame {
    fn result(&self) -> Result {
        match self {
            RockPaperScissorsGame {
                my_move: Rock,
                their_move: Scissors,
            }
            | RockPaperScissorsGame {
                my_move: Paper,
                their_move: Rock,
            }
            | RockPaperScissorsGame {
                my_move: Scissors,
                their_move: Paper,
            } => IWin,
            RockPaperScissorsGame {
                my_move: m,
                their_move: t,
            } if m == t => Draw,
            _ => TheyWin,
        }
    }

    fn score(&self) -> i32 {
        let outcome_score = match self.result() {
            IWin => 6,
            Draw => 3,
            TheyWin => 0,
        };

        outcome_score + self.my_move.score()
    }
}

fn parse_move(rps_move: &str) -> Move {
    match rps_move {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Nope"),
    }
}

fn parse_needed_result(rps_move: &str) -> Result {
    match rps_move {
        "X" => TheyWin,
        "Y" => Draw,
        "Z" => IWin,
        _ => panic!("Nope"),
    }
}

fn needed_move(their_move: Move, needed_result: Result) -> Move {
    match needed_result {
        Draw => their_move,
        IWin => their_move.beaten_by(),
        TheyWin => their_move.beats(),
    }
}

fn parse_line(l: &str) -> RockPaperScissorsGame {
    let mut iter = l.split_whitespace();
    let theirs = iter.next().unwrap();
    let mine = iter.next().unwrap();

    RockPaperScissorsGame {
        their_move: parse_move(theirs),
        my_move: needed_move(parse_move(theirs), parse_needed_result(mine)),
    }
}

fn main() {
    let bytes = include_bytes!("../input.txt");

    let z: i32 = String::from_utf8_lossy(bytes)
        .to_string()
        .lines()
        .map(|f| parse_line(f))
        //.for_each(|f| print!{"{:?}\n", f})
        .map(|g| g.score())
        //.for_each(|x| print!{"{:?}\n", x})
        .sum();

    print! {"The final sum is {}", z};
    // Part 1: 11873
    // Part 2: 12014
}
