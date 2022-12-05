#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day1;

impl Day1 {
    pub fn solve(name: &str) -> (i32, i32) {
        let c = read_file(name).unwrap();
        let mut cals = c
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .map(|l| l.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .collect::<Vec<i32>>();
        cals.sort();
        let top3: Vec<i32> = cals.into_iter().rev().take(3).collect();

        (
            *top3.clone().last().unwrap(),
            top3.clone().iter().sum::<i32>(),
        )
    }
}
