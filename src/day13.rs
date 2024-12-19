#![allow(unused)]
use std::{fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day13.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    let input = read_input();
    let mut line_iter = input.lines().peekable();

    let mut answer = 0;

    while line_iter.peek().is_some() {
        let (ax, ay) = parse_button(line_iter.next().unwrap());
        let (bx, by) = parse_button(line_iter.next().unwrap());
        let (prize_x, prize_y) = parse_prize(line_iter.next().unwrap());
        line_iter.next();
        if let Some(x) = solve((ax, ay), (bx, by), (prize_x, prize_y)) {
            answer += x;
        }
    }

    answer
}

fn parse_button(button_str: &str) -> (i64, i64) {
    let button_str = &button_str["Button A: ".len()..];
    let (x_part, y_part) = button_str.split_once(", ").unwrap();

    (
        x_part[2..].parse().unwrap(),
        y_part[2..y_part.len()].parse().unwrap(),
    )
}

fn parse_prize(prize_str: &str) -> (i64, i64) {
    let prize_str = &prize_str["Prize: ".len()..];
    let (x_part, y_part) = prize_str.split_once(", ").unwrap();

    (
        x_part[2..].parse().unwrap(),
        y_part[2..y_part.len()].parse().unwrap(),
    )
}

// NOTE: inputs are transposed from the buttons, i.e. button A is the .0 of the first two.
fn solve(
    (ax, bx): (i64, i64),
    (ay, by): (i64, i64),
    (prize_x, prize_y): (i64, i64),
) -> Option<i64> {
    let det = ax * by - bx * ay;

    if det != 0 {
        // compute inverse * det * prize and see if it's divisible by det
        // inverse is 1/det ( ( d, -b) (-c a))
        let ans_x = by * prize_x - ay * prize_y;
        let ans_y = -bx * prize_x + ax * prize_y;
        if ans_x % det == 0 && ans_y % det == 0 && ans_x * det >= 0 && ans_y * det >= 0 {
            return Some(3 * ans_x / det + ans_y / det);
        } else {
            return None;
        }
    }

    let using_a = if prize_x % ax == 0 && prize_y % ay == 0 {
        Some(prize_x / ax * 3 + prize_y / ay)
    } else {
        None
    };

    let using_b = if prize_x % bx == 0 && prize_y % by == 0 {
        Some(prize_x / bx * 3 + prize_y / by)
    } else {
        None
    };

    [using_a, using_b].iter().filter_map(|x| *x).max()
}

pub fn part2() -> i64 {
    let input = read_input();
    let mut line_iter = input.lines().peekable();

    let mut answer = 0;

    while line_iter.peek().is_some() {
        let (ax, ay) = parse_button(line_iter.next().unwrap());
        let (bx, by) = parse_button(line_iter.next().unwrap());
        let (prize_x, prize_y) = parse_prize(line_iter.next().unwrap());
        line_iter.next();
        if let Some(x) = solve(
            (ax, ay),
            (bx, by),
            (prize_x + 10000000000000, prize_y + 10000000000000),
        ) {
            answer += x;
        }
    }

    answer
}
