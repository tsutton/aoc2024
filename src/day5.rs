use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day5.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

struct Rules {
    befores: HashMap<i64, Vec<i64>>,
}

impl Rules {
    fn new() -> Self {
        Self {
            befores: HashMap::new(),
        }
    }

    fn add_rule(&mut self, before: i64, after: i64) {
        self.befores.entry(after).or_default().push(before);
    }

    fn is_before(&self, before: i64, after: i64) -> bool {
        self.befores
            .get(&after)
            .unwrap_or(&vec![])
            .contains(&before)
    }

    fn is_in_order(&self, values: &[i64]) -> bool {
        for i in 0..values.len() - 1 {
            if !self.is_before(values[i], values[i + 1]) {
                return false;
            }
        }
        true
    }
}

pub fn part1() -> i64 {
    let input = read_input();

    let lines: Vec<&str> = input.lines().collect();

    let separator = lines.iter().position(|line| line.is_empty()).unwrap();

    let mut rules = Rules::new();

    for rule in lines[..separator].iter() {
        let (left, right) = rule.split_once('|').unwrap();
        rules.add_rule(left.parse().unwrap(), right.parse().unwrap());
    }

    let mut answer = 0;

    for update in lines[separator + 1..].iter() {
        let values: Vec<i64> = update.split(',').map(|l| l.parse().unwrap()).collect();
        if rules.is_in_order(&values) {
            answer += values[(values.len() - 1) / 2]
        }
    }

    answer
}

pub fn part2() -> i64 {
    let input = read_input();

    let lines: Vec<&str> = input.lines().collect();

    let separator = lines.iter().position(|line| line.is_empty()).unwrap();

    let mut rules = Rules::new();

    for rule in lines[..separator].iter() {
        let (left, right) = rule.split_once('|').unwrap();
        rules.add_rule(left.parse().unwrap(), right.parse().unwrap());
    }

    let mut answer = 0;

    for update in lines[separator + 1..].iter() {
        let mut values: Vec<i64> = update.split(',').map(|l| l.parse().unwrap()).collect();
        if !rules.is_in_order(&values) {
            values.sort_by(|l, r| {
                if l == r {
                    Ordering::Equal
                } else if rules.is_before(*l, *r) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            answer += values[(values.len() - 1) / 2]
        }
    }

    answer
}
