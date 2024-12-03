use std::{fs::File, io::Read};

const EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day2.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn day2_part1() -> usize {
    let input = read_input();
    input.lines().filter(|line| is_safe(line)).count()
}

fn is_safe(s: &str) -> bool {
    let parts: Vec<i32> = s
        .split_ascii_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();
    if parts.len() <= 1 {
        return true;
    }
    if parts[1] == parts[0] {
        return false;
    }
    let is_increasing = parts[1] > parts[0];
    for i in 0..parts.len() - 1 {
        if is_increasing {
            if parts[i + 1] <= parts[i] || parts[i + 1] - parts[i] > 3 {
                return false;
            }
        } else if parts[i + 1] >= parts[i] || parts[i] - parts[i + 1] > 3 {
            return false;
        }
    }
    true
}

pub fn day2_part2() -> usize {
    let input = read_input();
    let i = input.lines().filter(|s| is_safe_part_2(s)).count();
    i
}

fn is_safe_part_2(s: &str) -> bool {
    let parts: Vec<i32> = s
        .split_ascii_whitespace()
        .map(|p| p.parse().unwrap())
        .collect();
    if parts.len() <= 1 {
        return true;
    }

    if parts.windows(2).all(|x| (1..=3).contains(&(x[1] - x[0]))) {
        return true;
    }
    if parts.windows(2).all(|x| (-3..=-1).contains(&(x[1] - x[0]))) {
        return true;
    }

    for skip_idx in 0..parts.len() {
        let new_parts: Vec<_> = (0..parts.len() - 1)
            .map(|i| if i < skip_idx { parts[i] } else { parts[i + 1] })
            .collect();
        if new_parts
            .windows(2)
            .all(|x| (1..=3).contains(&(x[1] - x[0])))
        {
            return true;
        }
        if new_parts
            .windows(2)
            .all(|x| (-3..=-1).contains(&(x[1] - x[0])))
        {
            return true;
        }
    }

    false
}
