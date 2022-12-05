#[path = "common.rs"]
mod common;

use common::read_file;
use std::collections::HashSet;

pub struct Day3;

fn common(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    let mut result = sets.pop().unwrap();
    result.retain(|item| sets.iter().all(|other| other.contains(item)));
    result
}

fn val(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 64 + 26,
        _ => 0,
    }
}

fn populate_set(s: &str, set: &mut HashSet<char>) {
    s.chars().for_each(|c| {
        set.insert(c);
    });
}

impl Day3 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let c = read_file(name).unwrap();

        let (mut s1, mut s2, mut s3): (HashSet<char>, HashSet<char>, HashSet<char>) =
            (HashSet::new(), HashSet::new(), HashSet::new());
        let mut letters: Vec<char> = Vec::new();
        let lines = c.split("\n").collect::<Vec<&str>>();

        for l in lines.clone() {
            assert!(l.len() % 2 == 0);
            assert!(s1.is_empty());
            assert!(s2.is_empty());

            let (start, end) = l.split_at(l.len() / 2);
            populate_set(start, &mut s1);
            populate_set(end, &mut s2);
            letters.push(s1.intersection(&s2).last()?.clone());
            s1.clear();
            s2.clear();
        }
        let r1 = letters.into_iter().map(val).sum();

        letters = Vec::new();
        for elf in lines.chunks(3) {
            let [e1, e2, e3]: [&str; 3] = elf.try_into().unwrap();
            assert!(s1.is_empty());
            assert!(s2.is_empty());
            assert!(s3.is_empty());

            populate_set(e1, &mut s1);
            populate_set(e2, &mut s2);
            populate_set(e3, &mut s3);
            let int = common(vec![s1.clone(), s2.clone(), s3.clone()]);
            s1.clear();
            s2.clear();
            s3.clear();

            letters.push(int.iter().take(1).next()?.clone());
        }
        let r2 = letters.into_iter().map(val).sum();

        Some((r1, r2))
    }
}
