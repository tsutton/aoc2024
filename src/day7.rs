use std::{fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day7.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    let input = read_input();
    input
        .lines()
        .map(parse_line)
        .filter(|(target, values)| attainable(*target, values))
        .map(|(target, _)| target)
        .sum()
}

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let (target, right) = line.split_once(':').unwrap();
    let values = right[" ".len()..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (target.parse().unwrap(), values)
}

fn attainable(target: i64, values: &[i64]) -> bool {
    if values.is_empty() {
        return false;
    } else if values.len() == 1 {
        return values[0] == target;
    }

    let last = *values.last().unwrap();
    if last > target {
        return false;
    }
    if attainable(target - last, &values[0..values.len() - 1]) {
        return true;
    }
    if target % last == 0 && attainable(target / last, &values[0..values.len() - 1]) {
        return true;
    }

    false
}

pub fn part2() -> i64 {
    let input = read_input();
    input
        .lines()
        .map(parse_line)
        .filter(|(target, values)| attainable_pt2(*target, values))
        .map(|(target, _)| target)
        .sum()
}

fn try_remove_suffix(base: i64, suffix: i64) -> Option<i64> {
    base.to_string().strip_suffix(&suffix.to_string()).map(|x| {
        if !x.is_empty() {
            x.parse().unwrap()
        } else {
            0
        }
    })
}

fn attainable_pt2(target: i64, values: &[i64]) -> bool {
    if values.is_empty() {
        return false;
    } else if values.len() == 1 {
        return values[0] == target;
    }

    let last = *values.last().unwrap();
    if last > target {
        return false;
    }
    if attainable_pt2(target - last, &values[0..values.len() - 1]) {
        return true;
    }
    if target % last == 0 && attainable_pt2(target / last, &values[0..values.len() - 1]) {
        return true;
    }

    if let Some(next_target) = try_remove_suffix(target, last) {
        if attainable_pt2(next_target, &values[0..values.len() - 1]) {
            return true;
        }
    }

    // if the target has a suffix of the last thing ...

    false
}
