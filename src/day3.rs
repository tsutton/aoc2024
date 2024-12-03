use std::{fs::File, io::Read};

use once_cell::sync::Lazy;
use regex::Regex;

const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day3.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}
pub fn part1() -> i64 {
    let input = read_input();
    let muls_regex = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    muls_regex
        .captures_iter(&input)
        .map(|capture| {
            capture.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * capture.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum()
}

struct Parser<'input> {
    input: &'input str,
}

impl Iterator for Parser<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("{}", self.input);
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^mul\((\d+),(\d+)\)"#).unwrap());

        if self.input.is_empty() {
            None
        } else if self.input.starts_with("do()") {
            self.input = &self.input["do()".len()..];
            Some(Instruction::Do)
        } else if self.input.starts_with("don't()") {
            self.input = &self.input["don't()".len()..];
            Some(Instruction::Dont)
        } else {
            if let Some(capture) = RE.captures(self.input) {
                self.input = &self.input[capture.len()..];
                Some(Instruction::Mul(
                    capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    capture.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ))
            } else {
                self.input = &self.input[1..];
                self.next()
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

struct Interpreter {
    enabled: bool,
}

pub fn part2() -> i64 {
    // let input = EXAMPLE;
    // let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    let input = &read_input();
    let parser = Parser { input };
    let mut interpreter = Interpreter { enabled: true };
    let mut ans = 0;
    for instr in parser {
        // println!("{:?}", instr);
        match instr {
            Instruction::Do => {
                interpreter.enabled = true;
            }
            Instruction::Dont => {
                interpreter.enabled = false;
            }
            Instruction::Mul(a, b) => {
                if interpreter.enabled {
                    ans += a * b;
                }
            }
        }
    }
    ans
}
