#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day5;

#[derive(Clone, Copy, Debug)]
enum Crate {
    Empty,
    Full(char),
}

fn parse_box(b: &str) -> Crate {
    if b.chars().nth(0).unwrap() != '[' {
        Crate::Empty
    } else {
        Crate::Full(b.chars().nth(1).unwrap())
    }
}

fn split_crates(line: &str) -> Vec<Crate> {
    let (bx, rest): (&str, &str);
    if line.len() < 4 {
        bx = line;
        rest = "";
    } else {
        (bx, rest) = line.split_at(4);
    }
    if rest.is_empty() {
        return vec![parse_box(bx)];
    } else {
        let mut a = vec![parse_box(bx)];
        a.append(&mut split_crates(rest));
        a
    }
}

impl Day5 {
    pub fn solve(name: &str) -> Option<(String, String)> {
        let c = read_file(name).unwrap();

        let (start, instructions) = c.split_once("\n\n")?;
        //let (start, mut instructions) = lines.split;
        let mut field: Vec<&str> = start.split("\n").collect();
        field.pop();

        let mut stacks: Vec<Vec<Crate>> = vec![Vec::new(); 9];
        field.into_iter().rev().map(split_crates).for_each(|level| {
            for (col, c) in level.into_iter().enumerate() {
                match c {
                    Crate::Empty => {}
                    Crate::Full(_) => stacks[col].push(c),
                }
            }
        });

        let mut stacks2 = stacks.clone();

        instructions.clone().split("\n").for_each(|i| {
            let a: Vec<&str> = i.split_whitespace().collect();
            let amt = a[1].parse::<usize>().unwrap();
            let from = a[3].parse::<usize>().unwrap();
            let to = a[5].parse::<usize>().unwrap();

            for _ in 0..amt {
                let tmp = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(tmp);
            }
        });
        let r1 = stacks
            .iter()
            .map(|col| {
                if let Crate::Full(v) = col.last().unwrap() {
                    v.clone()
                } else {
                    ' '
                }
            })
            .collect::<String>();

        instructions.clone().split("\n").for_each(|i| {
            let a: Vec<&str> = i.split_whitespace().collect();
            let amt = a[1].parse::<usize>().unwrap();
            let from = a[3].parse::<usize>().unwrap();
            let to = a[5].parse::<usize>().unwrap();

            let mut tmp_stack: Vec<Crate> = Vec::new();
            for _ in 0..amt {
                let tmp = stacks2[from - 1].pop().unwrap();
                tmp_stack.push(tmp);
            }
            tmp_stack.reverse();
            stacks2[to - 1].append(&mut tmp_stack);
        });
        let r2 = stacks2
            .iter()
            .map(|col| {
                if let Crate::Full(v) = col.last().unwrap() {
                    v.clone()
                } else {
                    ' '
                }
            })
            .collect::<String>();

        Some((r1, r2))
    }
}
