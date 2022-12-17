#[path = "common.rs"]
mod common;

use common::read_file;
use std::collections::HashSet;

pub struct Day9;

struct Rope {
    els: Vec<Point>,
    visited: HashSet<String>,
}

impl Rope {
    fn new(len: usize) -> Self {
        assert!(len > 1);
        Self {
            els: vec![Point::new(0, 0); len],
            visited: HashSet::new(),
        }
    }

    fn simulate(&mut self, mov: (i32, i32)) {
        self.els[0].mv(mov);
        // move elements one by one
        for i in 1..self.els.len() {
            let prev = self.els[i - 1];
            self.els[i].follow(&prev)
        }
        self.visited.insert(self.els.last().unwrap().coords());
    }

    fn visited(&self) -> i32 {
        self.visited.len() as i32
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Point) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    fn coords(&self) -> String {
        format!("{}:{}", self.x, self.y)
    }

    fn mv(&mut self, mov: (i32, i32)) {
        self.x += mov.0;
        self.y += mov.1;
    }

    fn follow(&mut self, head: &Point) {
        let d = head.dist(self);
        match d {
            (-2, 1) | (-1, 2) | (-2, 2) => self.mv((-1, 1)),
            (2, -1) | (1, -2) | (2, -2) => self.mv((1, -1)),
            (2, 1) | (1, 2) | (2, 2) => self.mv((1, 1)),
            (-2, -1) | (-1, -2) | (-2, -2) => self.mv((-1, -1)),
            (-2, 0) => self.mv((-1, 0)),
            (2, 0) => self.mv((1, 0)),
            (0, -2) => self.mv((0, -1)),
            (0, 2) => self.mv((0, 1)),
            _ => {}
        }
    }
}

fn decode(l: (&str, &str)) -> (i32, (i32, i32)) {
    let amt: i32 = l.1.parse().unwrap();
    match l.0 {
        "R" => (amt, (1, 0)),
        "L" => (amt, (-1, 0)),
        "U" => (amt, (0, 1)),
        "D" => (amt, (0, -1)),
        _ => unreachable!(),
    }
}

impl Day9 {
    pub fn solve(name: &str) -> Option<(i32, i32)> {
        let input = read_file(name).unwrap();
        let (mut short_rope, mut long_rope) = (Rope::new(2), Rope::new(10));
        input.split("\n").for_each(|cmd| {
            let (amt, mov) = decode(cmd.split_once(" ").unwrap());
            for _ in 0..amt {
                short_rope.simulate(mov);
                long_rope.simulate(mov);
            }
        });
        Some((short_rope.visited(), long_rope.visited()))
    }
}
