#[path = "common.rs"]
mod common;

use common::read_file;

pub struct Day10;

struct Processor {
    pc: i32,
    x: i32,
}

enum Instruction {
    ADD(i32),
    NOOP,
}

fn parse_instruction(i: &str) -> Instruction {
    if let Some((_, v)) = i.split_once(" ") {
        Instruction::ADD(v.parse().unwrap())
    } else {
        Instruction::NOOP
    }
}

impl Processor {
    fn new() -> Self {
        Processor { pc: 0, x: 1 }
    }

    fn increase_pc(&mut self) -> i32 {
        self.pc += 1;
        if (self.pc - 20) % 40 == 0 {
            return self.pc * self.x;
        }
        return 0;
    }

    fn run_p1(&mut self, ins: &Vec<&str>) -> i32 {
        let mut sum = 0;

        for i in ins {
            sum += self.increase_pc();
            match parse_instruction(i) {
                Instruction::ADD(v) => {
                    sum += self.increase_pc();
                    self.x += v;
                }
                Instruction::NOOP => {}
            }
        }
        sum
    }

    fn draw(&self) -> &str {
        let pos = (self.pc - 1) % 40;
        if pos == 39 {
            if (self.x - pos).abs() < 2 {
                return "#\n";
            } else {
                return ".\n";
            }
        }
        if (self.x - pos).abs() < 2 {
            "#"
        } else {
            "."
        }
    }

    fn run_p2(&mut self, ins: &Vec<&str>) -> String {
        let mut img = String::new();

        for i in ins {
            self.increase_pc();
            img += self.draw();
            match parse_instruction(i) {
                Instruction::ADD(v) => {
                    self.increase_pc();
                    img += self.draw();
                    self.x += v;
                }
                Instruction::NOOP => {}
            }
        }
        img
    }
}

impl Day10 {
    pub fn solve(name: &str) -> Option<(String, String)> {
        let input = read_file(name).unwrap();
        let inst: Vec<&str> = input.split("\n").collect();
        let mut proc = Processor::new();
        let r1 = proc.run_p1(&inst);
        let mut proc = Processor::new();
        //println!("{}", proc.run_p2(&inst));

        Some((r1.to_string(), format!("\n\n{}", proc.run_p2(&inst))))
    }
}
