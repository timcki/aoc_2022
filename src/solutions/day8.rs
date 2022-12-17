#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day8;

struct Map {
    trees: Vec<Vec<u32>>,
    size: usize,
}

impl Map {
    fn from_string(s: &String) -> Self {
        let lines = s.split("\n");
        let map: Vec<Vec<u32>> = lines
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Self {
            size: map.len(),
            trees: map,
        }
    }

    fn border_len(&self) -> i32 {
        self.size as i32 * 4 - 4
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        let height = self.trees[x][y];
        let (mut l, mut r, mut t, mut d) = (true, true, true, true);
        for a in (0..x).rev() {
            if height <= self.trees[a][y] {
                l = false;
                break;
            }
        }
        for a in x + 1..self.size {
            if height <= self.trees[a][y] {
                r = false;
                break;
            }
        }
        for a in (0..y).rev() {
            if height <= self.trees[x][a] {
                t = false;
                break;
            }
        }
        for a in y + 1..self.size {
            if height <= self.trees[x][a] {
                d = false;
                break;
            }
        }
        l || r || t || d
    }

    fn scenic(&self, x: usize, y: usize) -> i32 {
        let height = self.trees[x][y];
        let (mut l, mut r, mut t, mut d) = (0, 0, 0, 0);
        for a in (0..x).rev() {
            l += 1;
            if height <= self.trees[a][y] {
                break;
            }
        }
        for a in x + 1..self.size {
            r += 1;
            if height <= self.trees[a][y] {
                break;
            }
        }
        for a in (0..y).rev() {
            t += 1;
            if height <= self.trees[x][a] {
                break;
            }
        }
        for a in y + 1..self.size {
            d += 1;
            if height <= self.trees[x][a] {
                break;
            }
        }
        l * r * t * d
    }
}

impl Day8 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let input = read_file(name).unwrap();
        let map = Map::from_string(&input);
        let mut sum = map.border_len();
        let mut best = 0;

        for x in 1..map.size - 1 {
            for y in 1..map.size - 1 {
                if map.visible(x, y) {
                    sum += 1;
                }
                let s = map.scenic(x, y);
                if s > best {
                    best = s;
                }
            }
        }

        Some((sum as usize, best as usize))
    }
}
