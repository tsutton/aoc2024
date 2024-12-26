use std::{fmt::Display, fs::File, io::Read};

#[allow(dead_code)]
const SMALL_EXAMPLE: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;

#[allow(dead_code)]
const EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day15.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum WarehouseSpace {
    Empty,
    Box,
    Wall,
    Robot,
}

struct Warehouse {
    robot_x: usize,
    robot_y: usize,
    warehouse_grid: Vec<Vec<WarehouseSpace>>,
}

impl Warehouse {
    fn from_string(s: &str) -> Self {
        let mut warehouse_grid = Vec::new();

        let mut robot_x = 0;
        let mut robot_y = 0;

        for (r, line) in s.lines().enumerate() {
            let row = line
                .chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => WarehouseSpace::Empty,
                    '#' => WarehouseSpace::Wall,
                    'O' => WarehouseSpace::Box,
                    '@' => {
                        robot_x = r;
                        robot_y = col;
                        WarehouseSpace::Robot
                    }
                    _ => unreachable!(),
                })
                .collect();
            warehouse_grid.push(row);
        }
        Self {
            robot_x,
            robot_y,
            warehouse_grid,
        }
    }

    fn move_robot(&mut self, d: Direction) {
        let (next_x, next_y) = match d {
            Direction::Left => (self.robot_x - 1, self.robot_y),
            Direction::Right => (self.robot_x + 1, self.robot_y),
            Direction::Up => (self.robot_x, self.robot_y - 1),
            Direction::Down => (self.robot_x, self.robot_y + 1),
        };
        match self.warehouse_grid[next_y][next_x] {
            WarehouseSpace::Empty => {
                self.warehouse_grid[self.robot_y][self.robot_x] = WarehouseSpace::Empty;
                self.robot_x = next_x;
                self.robot_y = next_y;
                self.warehouse_grid[self.robot_y][self.robot_x] = WarehouseSpace::Robot;
            }
            WarehouseSpace::Box => {
                let (mut end_of_stack_x, mut end_of_stack_y) = (next_x, next_y);
                while !matches!(
                    self.warehouse_grid[end_of_stack_y][end_of_stack_x],
                    WarehouseSpace::Empty | WarehouseSpace::Wall
                ) {
                    (end_of_stack_x, end_of_stack_y) = match d {
                        Direction::Left => (end_of_stack_x - 1, end_of_stack_y),
                        Direction::Right => (end_of_stack_x + 1, end_of_stack_y),
                        Direction::Up => (end_of_stack_x, end_of_stack_y - 1),
                        Direction::Down => (end_of_stack_x, end_of_stack_y + 1),
                    };
                }
                if self.warehouse_grid[end_of_stack_y][end_of_stack_x] == WarehouseSpace::Empty {
                    self.warehouse_grid[self.robot_y][self.robot_x] = WarehouseSpace::Empty;
                    self.warehouse_grid[end_of_stack_y][end_of_stack_x] = WarehouseSpace::Box;
                    self.warehouse_grid[next_y][next_x] = WarehouseSpace::Robot;
                    self.robot_x = next_x;
                    self.robot_y = next_y;
                }
            }
            WarehouseSpace::Wall => (),
            WarehouseSpace::Robot => unreachable!(),
        }
    }

    fn score(&self) -> i64 {
        let mut sum = 0;
        for y in 0..self.warehouse_grid.len() {
            for x in 0..self.warehouse_grid[0].len() {
                if let WarehouseSpace::Box = self.warehouse_grid[y][x] {
                    sum += 100 * y + x;
                }
            }
        }
        sum.try_into().unwrap()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.warehouse_grid.iter() {
            for space in row.iter() {
                write!(
                    f,
                    "{}",
                    match space {
                        WarehouseSpace::Empty => '.',
                        WarehouseSpace::Box => 'O',
                        WarehouseSpace::Wall => '#',
                        WarehouseSpace::Robot => '@',
                    }
                )?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub fn part1() -> i64 {
    let s = read_input();
    let i = s.find("\n\n").unwrap();
    let mut w = Warehouse::from_string(&s[0..i]);
    println!("{}", w);

    let instructions = &s[i + 2..];
    for c in instructions.chars() {
        match c {
            '<' => w.move_robot(Direction::Left),
            '>' => w.move_robot(Direction::Right),
            '^' => w.move_robot(Direction::Up),
            'v' => w.move_robot(Direction::Down),
            _ => {}
        }
    }

    println!("{}", w);
    w.score()
}

pub fn part2() -> i64 {
    todo!()
}
