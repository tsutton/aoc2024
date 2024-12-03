const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

use std::{fs::File, io::Read};

use regex::Regex;

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
