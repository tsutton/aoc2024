use std::{collections::HashMap, fs::File, io::Read};

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day1.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn day1_part1() -> i32 {
    let input = read_input();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in input.lines() {
        let v: Vec<_> = line.split_ascii_whitespace().collect();
        left.push(v[0].parse().unwrap());
        right.push(v[1].parse().unwrap())
    }
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn day1_part2() -> i32 {
    let input = read_input();
    let mut left: Vec<i32> = vec![];
    let mut right: HashMap<i32, i32> = Default::default();
    for line in input.lines() {
        let v: Vec<_> = line.split_ascii_whitespace().collect();
        left.push(v[0].parse().unwrap());
        right
            .entry(v[1].parse().unwrap())
            .and_modify(|i| *i += 1)
            .or_insert(1);
    }
    println!("{:?}", right);

    left.iter().map(|x| x * right.get(x).unwrap_or(&0)).sum()
}
