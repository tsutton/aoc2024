use std::{fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day4.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    let input = read_input();
    // use as grid[row][col]
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row_idx in 0..rows {
        for col_idx in 0..cols {
            // horizontal or horizontal backwards

            if col_idx + "XMAS".len() <= cols {
                let horizontal_slice = &grid[row_idx][col_idx..col_idx + "XMAS".len()];
                if horizontal_slice == ['X', 'M', 'A', 'S'] {
                    println!("l-t-r {},{}", row_idx, col_idx);
                    count += 1
                }
                if horizontal_slice == ['S', 'A', 'M', 'X'] {
                    println!("r-t-l {},{}", row_idx, col_idx + 3);
                    count += 1
                }
            }
            // vertical or vertical backwards
            if row_idx + "XMAS".len() <= rows {
                let vertical_slice = [
                    grid[row_idx][col_idx],
                    grid[row_idx + 1][col_idx],
                    grid[row_idx + 2][col_idx],
                    grid[row_idx + 3][col_idx],
                ];
                if vertical_slice == ['X', 'M', 'A', 'S'] {
                    println!("t-to-b {},{}", row_idx, col_idx);
                    count += 1
                }
                if vertical_slice == ['S', 'A', 'M', 'X'] {
                    println!("t-to-b {},{}", row_idx + 3, col_idx);
                    count += 1
                }
            }
            if row_idx + "XMAS".len() <= rows && col_idx + "XMAS".len() <= cols {
                let diagonal_slice = [
                    grid[row_idx][col_idx],
                    grid[row_idx + 1][col_idx + 1],
                    grid[row_idx + 2][col_idx + 2],
                    grid[row_idx + 3][col_idx + 3],
                ];
                if diagonal_slice == ['X', 'M', 'A', 'S'] {
                    println!("diag down right {},{}", row_idx, col_idx);
                    count += 1
                }
                if diagonal_slice == ['S', 'A', 'M', 'X'] {
                    println!("diag up left {},{}", row_idx + 3, col_idx + 3);
                    count += 1
                }

                let up_diagonal_slice = [
                    grid[row_idx + 3][col_idx],
                    grid[row_idx + 2][col_idx + 1],
                    grid[row_idx + 1][col_idx + 2],
                    grid[row_idx][col_idx + 3],
                ];

                if up_diagonal_slice == ['X', 'M', 'A', 'S'] {
                    println!("diag up right {},{}", row_idx + 3, col_idx);
                    count += 1
                }
                if up_diagonal_slice == ['S', 'A', 'M', 'X'] {
                    println!("diag down left {},{}", row_idx, col_idx + 3);
                    count += 1
                }
            }
        }
    }

    count
}
pub fn part2() -> i64 {
    let input = read_input();
    // use as grid[row][col]
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row_idx in 1..rows - 1 {
        for col_idx in 1..cols - 1 {
            let forward_slash_slice = [
                grid[row_idx - 1][col_idx - 1],
                grid[row_idx][col_idx],
                grid[row_idx + 1][col_idx + 1],
            ];
            let backslash_slice = [
                grid[row_idx + 1][col_idx - 1],
                grid[row_idx][col_idx],
                grid[row_idx - 1][col_idx + 1],
            ];
            if (forward_slash_slice == ['M', 'A', 'S'] || forward_slash_slice == ['S', 'A', 'M'])
                && (backslash_slice == ['M', 'A', 'S'] || backslash_slice == ['S', 'A', 'M'])
            {
                count += 1;
            }
        }
    }

    count
}
