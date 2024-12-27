use std::{collections::HashMap, fs::File, io::Read};

use regex::Regex;

#[allow(dead_code)]
const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day19.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    let input = read_input();
    let lines: Vec<_> = input.lines().collect();
    let r = "^(".to_owned() + &lines[0].replace(", ", "|") + ")+$";
    let r = Regex::new(&r).unwrap();
    lines[2..]
        .iter()
        .filter(|l| r.is_match(l))
        .count()
        .try_into()
        .unwrap()
}

pub fn part2() -> i64 {
    let input = read_input();
    let lines: Vec<_> = input.lines().collect();
    let atoms: Vec<_> = lines[0].split(", ").collect();

    // lines[2..]
    //     .iter()
    //     .map(|line| count_decompositions(line, &atoms))
    //     .sum()
    let mut cache = HashMap::new();
    lines[2..]
        .iter()
        .map(|line| count_decompositions_with_cache(line, &atoms, &mut cache))
        .sum()
}

fn count_decompositions(target: &str, atoms: &[&str]) -> i64 {
    if target.is_empty() {
        return 1;
    }
    atoms
        .iter()
        .filter_map(|atom| {
            target
                .strip_prefix(atom)
                .map(|next_target| count_decompositions(next_target, atoms))
        })
        .sum()
}

fn count_decompositions_with_cache(
    target: &str,
    atoms: &[&str],
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if target.is_empty() {
        return 1;
    }
    if let Some(answer) = cache.get(target) {
        *answer
    } else {
        let answer = atoms
            .iter()
            .filter_map(|atom| {
                target
                    .strip_prefix(atom)
                    .map(|next_target| count_decompositions_with_cache(next_target, atoms, cache))
            })
            .sum();
        cache.insert(target.to_string(), answer);
        answer
    }
}
