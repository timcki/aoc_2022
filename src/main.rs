#![feature(concat_idents)]

mod solutions;

use solutions::{Day1, Day10, Day11, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9};
use std::{fmt::Display, io::Result};

fn solve_day<R: Display>(day: usize, solver: fn(&str) -> Option<(R, R)>) {
    let path = format!("inputs/{}.dat", day);
    let (r1, r2) = solver(&path).unwrap();
    println!("Day {} solution: {}, {}", day, r1, r2);
}

fn main() -> Result<()> {
    solve_day(1, Day1::solve);
    solve_day(2, Day2::solve);
    solve_day(3, Day3::solve);
    solve_day(4, Day4::solve);
    solve_day(5, Day5::solve);
    solve_day(6, Day6::solve);
    solve_day(7, Day7::solve);
    solve_day(8, Day8::solve);
    solve_day(9, Day9::solve);
    solve_day(10, Day10::solve);
    solve_day(11, Day11::solve);

    Ok(())
}
