use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

#[allow(dead_code)]
const EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day8.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    let input = read_input();
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push((row.try_into().unwrap(), col.try_into().unwrap()));
            }
        }
        width = line.len() as i64;
        height = (row + 1) as i64;
    }

    // println!("width {} height {}", width, height);
    // println!("{:?}", antennas);
    let mut antinodes = HashSet::new();
    for locs in antennas.values() {
        for first_idx in 0..locs.len() {
            for second_idx in first_idx + 1..locs.len() {
                let first = locs[first_idx];
                let second = locs[second_idx];
                let delta = (second.0 - first.0, second.1 - first.1);
                let tries = [
                    (second.0 + delta.0, second.1 + delta.1),
                    (first.0 - delta.0, first.1 - delta.1),
                ];
                for t in tries {
                    // println!("trying {:?} vs {:?} => {:?}", first, second, t);
                    if (0..width).contains(&t.1) && (0..height).contains(&t.0) {
                        // println!("check");
                        antinodes.insert(t);
                    }
                }
            }
        }
    }
    antinodes.len().try_into().unwrap()
}

pub fn part2() -> i64 {
    let input = read_input();
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push((row.try_into().unwrap(), col.try_into().unwrap()));
            }
        }
        width = line.len() as i64;
        height = (row + 1) as i64;
    }

    // println!("width {} height {}", width, height);
    // println!("{:?}", antennas);
    let mut antinodes = HashSet::new();
    for locs in antennas.values() {
        for first_idx in 0..locs.len() {
            for second_idx in first_idx + 1..locs.len() {
                let first = locs[first_idx];
                let second = locs[second_idx];
                let delta = lowest_terms(second.0 - first.0, second.1 - first.1);
                for i in 0.. {
                    let t = (first.0 - i * delta.0, first.1 - i * delta.1);
                    if (0..width).contains(&t.1) && (0..height).contains(&t.0) {
                        antinodes.insert(t);
                    } else {
                        break;
                    }
                }
                for i in 1.. {
                    let t = (first.0 + i * delta.0, first.1 + i * delta.1);
                    if (0..width).contains(&t.1) && (0..height).contains(&t.0) {
                        antinodes.insert(t);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len().try_into().unwrap()
}

fn lowest_terms(a: i64, b: i64) -> (i64, i64) {
    let originals = (a, b);
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    (originals.0 / a, originals.1 / a)
}
