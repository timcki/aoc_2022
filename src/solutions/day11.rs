#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day11;

struct Monkey {
    items: Vec<i32>,
    op: Box<dyn FnOnce(i32) -> i32>,
    test: Box<dyn FnOnce(i32) -> i32>,
}

fn take_second(l: &str) -> &str {
    let (_, v) = l.split_once(": ").unwrap();
    v.trim()
}

fn gen_op(sign: &str, value: &str) -> Box<dyn FnOnce(i32) -> i32> {
    let val = if let Ok(v) = value.parse::<i32>() {
        v
    } else {
        -1
    };

    if sign == "*" {
        Box::new(|old| if val == -1 { old * old } else { old * val })
    } else {
        Box::new(|old| if val == -1 { old + old } else { old + val })
    }
}

fn gen_test(sign: &str, value: &str) -> Box<dyn FnOnce(i32) -> i32> {
    let val = if let Ok(v) = value.parse::<i32>() {
        v
    } else {
        -1
    };

    if sign == "*" {
        Box::new(|old| if val == -1 { old * old } else { old * val })
    } else {
        Box::new(|old| if val == -1 { old + old } else { old + val })
    }
}

impl Monkey {
    fn from_string(input: &str) -> Self {
        let rules: Vec<&str> = input.split("\n").skip(1).collect();

        let items: Vec<i32> = take_second(rules[0])
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let (sign, value) = take_second(rules[1])
            .split_once("= old ")
            .unwrap()
            .1
            .split_once(" ")
            .unwrap();

        let op = gen_op(sign, value);
    }

    fn new(
        items: Vec<i32>,
        op: Box<dyn FnOnce(i32) -> i32>,
        test: Box<dyn FnOnce(i32) -> i32>,
    ) -> Self {
        Self { items, op, test }
    }
}

impl Day11 {
    pub fn solve(name: &str) -> Option<(i32, i32)> {
        let input = read_file(name).unwrap();
        let inst: Vec<&str> = input.split("\n\n").collect();
        for m in inst {
            println!("{}", m);
        }

        Some((0, 0))
    }
}
