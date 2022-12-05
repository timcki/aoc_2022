mod common;
mod day1;
mod day3;
mod day4;
mod day5;

use std::io;

use day1::Day1;
use day3::Day3;
use day4::Day4;
use day5::Day5;

fn main() -> io::Result<()> {
    print!("Solving day 1: ");
    let (r1, r2) = Day1::solve("inputs/1_day.dat");
    println!("{}, {}", r1, r2);

    print!("Solving day 3: ");
    let (r1, r2) = Day3::solve("inputs/3_day.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 4: ");
    let (r1, r2) = Day4::solve("inputs/4_day.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 5: ");
    let (r1, r2) = Day5::solve("inputs/5_day.dat").unwrap();
    println!("{}, {}", r1, r2);
    Ok(())
}
