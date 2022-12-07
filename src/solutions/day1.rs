#[path = "common.rs"]
mod common;

use common::{read_file, Solver};

pub struct Day1;

impl Day1 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let c = read_file(name).unwrap();
        let mut cals = c
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .map(|l| l.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect::<Vec<usize>>();
        cals.sort();
        let top3: Vec<usize> = cals.into_iter().rev().take(3).collect();

        Some((*top3.last().unwrap(), top3.iter().sum::<usize>()))
    }
}
