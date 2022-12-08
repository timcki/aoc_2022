#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day2;

#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn decode(c: &str) -> Option<Choice> {
    match c {
        "A" | "X" => Some(Choice::Rock),
        "B" | "Y" => Some(Choice::Paper),
        "C" | "Z" => Some(Choice::Scisors),
        _ => None,
    }
}

fn decode_outcome(c: &str) -> Option<Outcome> {
    match c {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn round_outcome(op: Choice, y: Choice) -> Outcome {
    match (op, y) {
        (Choice::Rock, Choice::Rock)
        | (Choice::Scisors, Choice::Scisors)
        | (Choice::Paper, Choice::Paper) => Outcome::Draw,
        (Choice::Rock, Choice::Paper)
        | (Choice::Paper, Choice::Scisors)
        | (Choice::Scisors, Choice::Rock) => Outcome::Win,
        (Choice::Rock, Choice::Scisors)
        | (Choice::Paper, Choice::Rock)
        | (Choice::Scisors, Choice::Paper) => Outcome::Lose,
    }
}

fn choose_for_outcome(op: Choice, res: Outcome) -> Choice {
    match (op, res) {
        (Choice::Rock, Outcome::Lose) => Choice::Scisors,
        (Choice::Rock, Outcome::Draw) => Choice::Rock,
        (Choice::Rock, Outcome::Win) => Choice::Paper,
        (Choice::Paper, Outcome::Lose) => Choice::Rock,
        (Choice::Paper, Outcome::Draw) => Choice::Paper,
        (Choice::Paper, Outcome::Win) => Choice::Scisors,
        (Choice::Scisors, Outcome::Lose) => Choice::Paper,
        (Choice::Scisors, Outcome::Draw) => Choice::Scisors,
        (Choice::Scisors, Outcome::Win) => Choice::Rock,
    }
}

impl Day2 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let c = read_file(name).unwrap();
        let lines = c.split("\n");

        let sum: usize = lines
            .clone()
            .map(|l| {
                let (op, y) = l.split_once(" ").unwrap();
                decode(y).unwrap() as usize
                    + round_outcome(decode(op).unwrap(), decode(y).unwrap()) as usize
            })
            .sum();

        let sum2 = lines
            .map(|l| {
                let (op, out) = l.split_once(" ").unwrap();
                let outcome = decode_outcome(out).unwrap();
                outcome as usize + choose_for_outcome(decode(op).unwrap(), outcome) as usize
            })
            .sum::<usize>();

        Some((sum, sum2))
    }
}
