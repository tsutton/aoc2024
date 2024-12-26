use std::{collections::BTreeMap, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day17.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>,
    ip: usize,
}

impl Computer {
    fn step(&mut self) -> (Option<usize>, bool) {
        if self.ip >= self.program.len() {
            return (None, false);
        }
        match self.program[self.ip] {
            0 => {
                // adv
                let numerator = self.reg_a;
                let denominator = 2usize.pow(
                    self.combo_operand_value(self.program[self.ip + 1])
                        .try_into()
                        .unwrap(),
                );
                self.reg_a = numerator / denominator;
                self.ip += 2;
            }
            1 => {
                // bxl
                self.reg_b ^= self.program[self.ip + 1];
                self.ip += 2;
            }
            2 => {
                // bst
                let base = self.combo_operand_value(self.program[self.ip + 1]);
                self.reg_b = base % 8;
                self.ip += 2;
            }
            3 => {
                // jnz
                if self.reg_a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = self.program[self.ip + 1];
                }
            }
            4 => {
                // bxc
                self.reg_b ^= self.reg_c;
                self.ip += 2;
            }
            5 => {
                // out
                self.ip += 2;
                return (
                    Some(self.combo_operand_value(self.program[self.ip - 1]) % 8),
                    true,
                );
            }
            6 => {
                // bdv
                let numerator = self.reg_a;
                let denominator = 2usize.pow(
                    self.combo_operand_value(self.program[self.ip + 1])
                        .try_into()
                        .unwrap(),
                );
                self.reg_b = numerator / denominator;
                self.ip += 2;
            }
            7 => {
                // bdv
                let numerator = self.reg_a;
                let denominator = 2usize.pow(
                    self.combo_operand_value(self.program[self.ip + 1])
                        .try_into()
                        .unwrap(),
                );
                self.reg_c = numerator / denominator;
                self.ip += 2;
            }
            _ => unreachable!(),
        }
        (None, true)
    }

    fn combo_operand_value(&self, operand: usize) -> usize {
        match operand {
            x @ 0..=3 => x,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            z => panic!("unexpected combo operand {z}"),
        }
    }

    fn run_to_completion(&mut self) -> Vec<usize> {
        let mut answer = vec![];
        while let (a, true) = self.step() {
            if let Some(b) = a {
                answer.push(b);
            }
        }
        answer
    }

    fn next_output(&mut self) -> Option<usize> {
        loop {
            match self.step() {
                (Some(a), _) => return Some(a),
                (None, true) => {}
                (_, false) => return None,
            }
        }
    }
}

pub fn part1() -> i64 {
    // Example
    let mut c = Computer {
        reg_a: 729,
        reg_b: 0,
        reg_c: 0,
        program: vec![0, 1, 5, 4, 3, 0],
        ip: 0,
    };

    // Actual
    // let mut c = Computer {
    //     reg_a: 28066687,
    //     reg_b: 0,
    //     reg_c: 0,
    //     program: vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0],
    //     ip: 0,
    // };
    while let (a, true) = c.step() {
        if let Some(b) = a {
            print!("{},", b);
        }
    }

    println!("==");
    0
}

pub fn part2() -> i64 {
    let program = vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0];

    /* This program is:

    2, 4,
    1, 1,
    7, 5,
    4, 6,
    0, 3,
    1, 4,
    5, 5,
    3, 0

    Bst: B = A % 8
    Bxl: B = B ^ 1: (flip the last bit)
    Cdv: C = A / 2^B
    Bxc: B = B ^ C
    Adv: A = A / 8
    Bxl: B = B ^ 4
    Out B
    if A != 0, back to beginning

    In particular, it has the property that the first output depends on all of A, but all of the next outputs only depend on A/8
    As such, we can start from the /end/: Find the a from 0 to 7 that outputs the last number we want, then build a up as octal

    So say the value a=7 outputs the final 0 of the program.
    Then we look for an octal number 0b7x, and find x. Then we look for an octal number 0b7xy, and find y (given we just computed x)

    */

    let mut possible_a = BTreeMap::new();
    // map of (value: number of correct outputs)
    // we'll always take the smallest value, find all possible next octal digits, then push those on
    possible_a.insert(0, 0);

    loop {
        let (a, correct_outputs) = possible_a.pop_first().unwrap();
        if correct_outputs == program.len() {
            break a.try_into().unwrap();
        }
        for next_digit in 0..=7 {
            let mut c = Computer {
                reg_a: a * 8 + next_digit,
                reg_b: 0,
                reg_c: 0,
                program: program.clone(),
                ip: 0,
            };
            let u = c.next_output().unwrap();
            if u == program[program.len() - correct_outputs - 1] {
                possible_a.insert(8 * a + next_digit, correct_outputs + 1);
            }
        }
    }
}
