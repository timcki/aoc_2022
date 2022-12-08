mod solutions;

use solutions::{Day1, Day2, Day3, Day4, Day5, Day6, Day7};
use std::io::Result;

fn main() -> Result<()> {
    print!("Solving day 1: ");
    let (r1, r2) = Day1::solve("inputs/1.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 2: ");
    let (r1, r2) = Day2::solve("inputs/2.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 3: ");
    let (r1, r2) = Day3::solve("inputs/3.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 4: ");
    let (r1, r2) = Day4::solve("inputs/4.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 5: ");
    let (r1, r2) = Day5::solve("inputs/5.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 6: ");
    let (r1, r2) = Day6::solve("inputs/6.dat").unwrap();
    println!("{}, {}", r1, r2);

    print!("Solving day 7: ");
    let (r1, r2) = Day7::solve("inputs/7.dat").unwrap();
    println!("{}, {}", r1, r2);
    Ok(())
}
