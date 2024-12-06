use std::{collections::HashSet, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day6.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Lab {
    obstacles: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    guard_direction: Direction,
    guard_location: (usize, usize),
}

impl Lab {
    fn new(input: &str) -> Self {
        let mut obstacles = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        let mut guard_direction = Direction::Up;
        let mut guard_location = (0, 0);
        for (row, line) in input.lines().enumerate() {
            if width == 0 {
                width = line.len()
            }
            height = row + 1;
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((row, col));
                    }
                    'v' => {
                        guard_direction = Direction::Down;
                        guard_location = (row, col);
                    }
                    '^' => {
                        guard_direction = Direction::Up;
                        guard_location = (row, col);
                    }
                    '>' => {
                        guard_direction = Direction::Right;
                        guard_location = (row, col);
                    }
                    '<' => {
                        guard_direction = Direction::Left;
                        guard_location = (row, col);
                    }
                    _ => {}
                }
            }
        }
        Self {
            obstacles,
            width,
            height,
            guard_direction,
            guard_location,
        }
    }

    fn step(&mut self) -> bool {
        let location: (i64, i64) = (
            self.guard_location.0.try_into().unwrap(),
            self.guard_location.1.try_into().unwrap(),
        );

        let direction_vector = match self.guard_direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let next_location = (
            location.0 + direction_vector.0,
            location.1 + direction_vector.1,
        );

        if next_location.0 < 0
            || next_location.1 < 0
            || next_location.0 >= self.height.try_into().unwrap()
            || next_location.1 >= self.width.try_into().unwrap()
        {
            return true;
        }

        let location_usizes: (usize, usize) = (
            next_location.0.try_into().unwrap(),
            next_location.1.try_into().unwrap(),
        );

        if self.obstacles.contains(&location_usizes) {
            match self.guard_direction {
                Direction::Up => self.guard_direction = Direction::Right,
                Direction::Down => self.guard_direction = Direction::Left,
                Direction::Left => self.guard_direction = Direction::Up,
                Direction::Right => self.guard_direction = Direction::Down,
            }
            self.step()
        } else {
            self.guard_location = location_usizes;
            false
        }
    }

    fn is_loop(&mut self) -> bool {
        let mut visited = HashSet::new();
        visited.insert((self.guard_location, self.guard_direction));
        while !self.step() {
            if visited.contains(&(self.guard_location, self.guard_direction)) {
                return true;
            } else {
                visited.insert((self.guard_location, self.guard_direction));
            }
        }

        false
    }
}

pub fn part1() -> i64 {
    let input = &read_input();
    let mut lab = Lab::new(input);
    let mut visited = 1;
    let mut grid = vec![vec!['.'; lab.width]; lab.height];
    grid[lab.guard_location.0][lab.guard_location.1] = 'X';
    while !lab.step() {
        if grid[lab.guard_location.0][lab.guard_location.1] != 'X' {
            visited += 1;
        }
        grid[lab.guard_location.0][lab.guard_location.1] = 'X';
    }
    println!(
        "{}",
        grid.iter()
            .map(|chars| chars.iter().collect::<String>())
            .fold(String::new(), |mut acc, next| {
                acc += &next;
                acc += "\n";
                acc
            })
    );
    visited
}

pub fn part2() -> i64 {
    let input = &read_input();

    let lab = Lab::new(input);

    let mut path_lab = lab.clone();
    // let mut grid = vec![vec!['.'; lab.width]; lab.height];
    let mut path = vec![];
    while !path_lab.step() {
        path.push((path_lab.guard_location.0, path_lab.guard_location.1));
    }

    let mut loop_spots = HashSet::new();
    for path_spot in path {
        if path_spot == lab.guard_location {
            continue;
        }
        let mut test_lab = lab.clone();
        test_lab.obstacles.insert(path_spot);

        if test_lab.is_loop() {
            println!("{:?}", path_spot);
            loop_spots.insert(path_spot);
        }
    }
    // First run the loop from part 1: the obstacle should go somewhere that the guard visits without changes
    // then loop over the places visited and try them as an obstacle, I guess (brute force)
    loop_spots.len().try_into().unwrap()
}

// (6,3), (7,6), (7,7), (8,1), (8,3), (9,7)
