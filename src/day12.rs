#![allow(unused)]
use std::{collections::HashSet, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"AAAA
BBCD
BBCC
EEEC
"#;

const EXAMPLE_BIG: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day12.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    let input = &read_input();
    let grid = gridify(input);

    let regions = find_regions(&grid);

    regions
        .iter()
        .map(|region| perimeter(region) * (i64::try_from(region.len()).unwrap()))
        .sum()
}

fn gridify(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_regions(grid: &[Vec<char>]) -> Vec<HashSet<(usize, usize)>> {
    let mut regions = vec![];

    let mut copy: Vec<Vec<char>> = grid.to_vec();

    let rows = copy.len();
    let cols = copy[0].len();

    loop {
        // find something unvisited
        // we could track visited and unvisited in hashsets to optimize but eh
        let z = copy
            .iter()
            .map(|row| row.iter().position(|&c| c != '.'))
            .enumerate()
            .find(|(_, opt)| opt.is_some());
        let Some((r, Some(c))) = z else { break };

        let target_char = copy[r][c];
        copy[r][c] = '.';

        let mut region = HashSet::from_iter([(r, c)]);
        let mut queue = vec![(r, c)];
        while let Some((r, c)) = queue.pop() {
            let mut good_neighbors = vec![];
            if r > 0 && copy[r - 1][c] == target_char {
                good_neighbors.push((r - 1, c));
            }
            if c > 0 && copy[r][c - 1] == target_char {
                good_neighbors.push((r, c - 1));
            }
            if r + 1 < rows && copy[r + 1][c] == target_char {
                good_neighbors.push((r + 1, c));
            }
            if c + 1 < cols && copy[r][c + 1] == target_char {
                good_neighbors.push((r, c + 1));
            }
            for (r1, c1) in good_neighbors {
                region.insert((r1, c1));
                copy[r1][c1] = '.';
                queue.push((r1, c1));
            }
        }

        regions.push(region);
    }

    regions
}

fn perimeter(region: &HashSet<(usize, usize)>) -> i64 {
    // each element in the region contributed based on which of its neighbors are in the region
    // for example, a square where the square above it is not in the region contributes its top edge

    let mut perim = 0;
    for &(row, col) in region {
        // check above
        if (row == 0) || !region.contains(&(row - 1, col)) {
            perim += 1;
        }
        // check below
        if !region.contains(&(row + 1, col)) {
            perim += 1;
        }

        // check left
        if col == 0 || !region.contains(&(row, col - 1)) {
            perim += 1;
        }

        // check right
        if !region.contains(&(row, col + 1)) {
            perim += 1;
        }
    }
    perim
}

pub fn part2() -> i64 {
    todo!()
}
