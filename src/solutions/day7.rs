#[path = "common.rs"]
mod common;

use common::read_file;
use std::collections::HashMap;

pub struct Day7;

const TOTAL_SPACE: usize = 70_000_000;
const FREE_SPACE: usize = 30_000_000;

// We assume / is the top dir and doesn't need to be in a hashmap itself
#[derive(Clone, Debug)]
struct Directory<'a> {
    name: &'a str,
    directories: HashMap<&'a str, Directory<'a>>,
    files: HashMap<&'a str, usize>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Directory {
            name,
            directories: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn get_dir(&mut self, path: &Vec<&str>) -> Option<&mut Directory<'a>> {
        match path.len() {
            0 => Some(self),
            1 => self.directories.get_mut(path[0]),
            _ => self
                .directories
                .get_mut(path[0])?
                .get_dir(&path[1..].to_vec()),
        }
    }

    fn size(&self) -> usize {
        let recursed: usize = self.directories.iter().map(|(_, dir)| dir.size()).sum();
        let this: usize = self.files.iter().map(|(_, s)| s).sum();

        recursed + this
    }

    fn sizes_by_dir(&self) -> HashMap<String, usize> {
        if self.directories.len() == 0 {
            let mut h = HashMap::new();
            h.insert(self.name.to_owned(), self.size());
            return h;
        }

        let dirs = self
            .directories
            .iter()
            .map(|(_, dir)| dir.sizes_by_dir())
            .collect::<Vec<HashMap<String, usize>>>();

        let mut ret = HashMap::new();
        ret.insert(self.name.to_owned(), self.size());
        for dir in dirs {
            dir.iter().for_each(|(k, v)| {
                ret.insert((self.name.to_owned() + "/") + &k, v.clone());
            })
        }
        ret
    }
}

#[derive(Clone, Copy, Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}

fn parse_command(cmd: &str) -> Command {
    if let Some((_, name)) = cmd.split_once(" ") {
        Command::Cd(name)
    } else {
        Command::Ls
    }
}

impl Day7 {
    pub fn solve(name: &str) -> Option<(usize, usize)> {
        let input = read_file(name).unwrap();
        let mut directories = Directory::new("/");
        let mut path: Vec<&str> = Vec::new();

        let commands = input.split("$ ").into_iter().skip(2);
        for cmd in commands {
            let c = cmd.split("\n").collect::<Vec<&str>>();
            match parse_command(c.first()?) {
                Command::Cd(dirname) => {
                    if dirname == ".." {
                        path.pop();
                    } else {
                        path.push(dirname)
                    }
                }
                Command::Ls => c[1..].into_iter().for_each(|line| {
                    if let Some((f, s)) = line.clone().split_once(" ") {
                        // If we listed a dir insert it
                        if f == "dir" {
                            if let Some(dir) = directories.get_dir(&path) {
                                dir.directories.insert(s, Directory::new(s));
                            }
                        } else {
                            if let Some(dir) = directories.get_dir(&path) {
                                dir.files.insert(s, f.parse::<usize>().unwrap());
                            }
                        }
                    }
                }),
            }
        }
        let dir_size = directories.sizes_by_dir();
        let r1 = dir_size
            .iter()
            .filter_map(|(_, v)| if *v < 100_000 { Some(v) } else { None })
            .sum::<usize>();

        let used_size = directories.size();
        let free_size: i32 = (TOTAL_SPACE - used_size) as i32;
        let needed = (free_size - FREE_SPACE as i32).abs();

        let mut x: Vec<i32> = dir_size.iter().map(|(_, v)| *v as i32).collect();
        x.sort();

        let r2 = x
            .iter()
            .rev()
            .reduce(|acc, v| if *v >= needed && acc >= v { v } else { acc })?;

        Some((r1, *r2 as usize))
    }
}
