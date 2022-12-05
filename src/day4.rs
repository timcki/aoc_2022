#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day4;

fn parse_range(r: &str) -> (usize, usize) {
    let (f, e) = r.split_once("-").unwrap();
    (f.parse().unwrap(), e.parse().unwrap())
}

fn check_ranges(t: (&str, &str)) -> usize {
    let (x, y) = parse_range(t.0);
    let (a, b) = parse_range(t.1);
    if (a >= x && b <= y) || (x >= a && y <= b) {
        1
    } else {
        0
    }
}

fn check_ranges_overlap(t: (&str, &str)) -> usize {
    let (sa, ea) = parse_range(t.0);
    let (sb, eb) = parse_range(t.1);
    if sa <= eb && ea >= sb {
        1
    } else {
        0
    }
}

impl Day4 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let c = read_file(name).unwrap();

        let lines = c.split("\n").collect::<Vec<&str>>();
        let r1 = lines
            .clone()
            .iter()
            .map(|l| l.split_once(",").unwrap())
            .map(check_ranges)
            .sum();

        let r2 = lines
            .iter()
            .map(|l| l.split_once(",").unwrap())
            .map(check_ranges_overlap)
            .sum();

        Some((r1, r2))
    }
}
