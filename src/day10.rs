use std::{collections::HashMap, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day10.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    // let input = EXAMPLE;
    let input = read_input();

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut answer = 0;
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            if grid[row_idx][col_idx] == 0 {
                answer += trailhead_score(&grid, row_idx, col_idx)
            }
        }
    }

    answer
}

fn trailhead_score(grid: &[Vec<u32>], start_row: usize, start_col: usize) -> i64 {
    let mut next_locs = vec![(start_row, start_col)];
    let mut next_height = 1;

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    while next_height <= 9 {
        let mut new_next_locs = vec![];
        for &(row, col) in next_locs.iter() {
            if row > 0 && grid[row - 1][col] == next_height {
                new_next_locs.push((row - 1, col));
            }
            if col > 0 && grid[row][col - 1] == next_height {
                new_next_locs.push((row, col - 1));
            }
            if row + 1 < num_rows && grid[row + 1][col] == next_height {
                new_next_locs.push((row + 1, col));
            }
            if col + 1 < num_cols && grid[row][col + 1] == next_height {
                new_next_locs.push((row, col + 1));
            }
        }

        new_next_locs.sort();
        new_next_locs.dedup();

        next_locs = new_next_locs;
        next_height += 1
    }

    // println!("for start ({start_row} {start_col}) found {next_locs:?}");

    next_locs.len().try_into().unwrap()
}

pub fn part2() -> i64 {
    // let input = EXAMPLE;
    let input = read_input();

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut answer = 0;
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            if grid[row_idx][col_idx] == 0 {
                answer += trailhead_score_pt2(&grid, row_idx, col_idx)
            }
        }
    }

    answer
}

fn trailhead_score_pt2(grid: &[Vec<u32>], start_row: usize, start_col: usize) -> i64 {
    let mut next_locs_with_count: HashMap<(usize, usize), i64> = HashMap::new();
    *next_locs_with_count
        .entry((start_row, start_col))
        .or_default() += 1;
    let mut next_height = 1;

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    while next_height <= 9 {
        let mut new_next_locs_with_count = HashMap::new();

        for (&(row, col), &score) in next_locs_with_count.iter() {
            if row > 0 && grid[row - 1][col] == next_height {
                *new_next_locs_with_count.entry((row - 1, col)).or_default() += score;
            }
            if col > 0 && grid[row][col - 1] == next_height {
                *new_next_locs_with_count.entry((row, col - 1)).or_default() += score;
            }
            if row + 1 < num_rows && grid[row + 1][col] == next_height {
                *new_next_locs_with_count.entry((row + 1, col)).or_default() += score;
            }
            if col + 1 < num_cols && grid[row][col + 1] == next_height {
                *new_next_locs_with_count.entry((row, col + 1)).or_default() += score;
            }
        }

        next_locs_with_count = new_next_locs_with_count;
        next_height += 1
    }

    // println!("for start ({start_row} {start_col}) found {next_locs_with_count:?}");

    next_locs_with_count.values().sum()
}
