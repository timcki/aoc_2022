#[path = "common.rs"]
mod common;

use std::collections::HashSet;

use common::read_file;

pub struct Day6;

fn find_window(sig: &String, size: usize, set: &mut HashSet<char>) -> usize {
    for (i, window) in sig.chars().collect::<Vec<char>>().windows(size).enumerate() {
        assert!(set.is_empty());
        let l = window
            .into_iter()
            .filter(|l| set.insert(l.to_owned().clone()))
            .map(|l| l.clone())
            .collect::<Vec<char>>()
            .len();
        set.clear();
        if l == size {
            return i + size;
        }
    }
    0
}

impl Day6 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let c = read_file(name).unwrap();

        let mut set: HashSet<char> = HashSet::new();
        let r1 = find_window(&c, 4, &mut set);
        let r2 = find_window(&c, 14, &mut set);

        Some((r1, r2))
    }
}
