use std::{collections::HashMap, fmt::Write, fs::File, io::Read};

#[allow(dead_code)]
const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Robot {
    x: i64,
    y: i64,

    vx: i64,
    vy: i64,
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let (position_part, velocity_part) = s.split_once(' ').unwrap();

        let (pos_x, pos_y) = position_part["p=".len()..].split_once(',').unwrap();
        let (vel_x, vel_y) = velocity_part["v=".len()..].split_once(',').unwrap();
        Self {
            x: pos_x.parse().unwrap(),
            y: pos_y.parse().unwrap(),
            vx: vel_x.parse().unwrap(),
            vy: vel_y.parse().unwrap(),
        }
    }

    fn advance(&mut self, steps: i64, x_size: i64, y_size: i64) {
        self.x = (self.vx * (steps % x_size) + self.x).rem_euclid(x_size);
        self.y = (self.vy * (steps % y_size) + self.y).rem_euclid(y_size);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Axis,
}

impl Robot {
    fn quadrant(&self, x_size: i64, y_size: i64) -> Quadrant {
        let center_x = (x_size - 1) / 2;
        let center_y = (y_size - 1) / 2;
        if self.x == center_x || self.y == center_y {
            Quadrant::Axis
        } else if self.x < center_x {
            return if self.y < center_y {
                Quadrant::TopLeft
            } else {
                Quadrant::BottomLeft
            };
        } else {
            return if self.y < center_y {
                Quadrant::TopRight
            } else {
                Quadrant::BottomRight
            };
        }
    }
}

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day14.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

pub fn part1() -> i64 {
    // let input = EXAMPLE;
    // let x_size = 11;
    // let y_size = 7;
    let input = read_input();
    let x_size = 101;
    let y_size = 103;
    let steps = 100;

    let mut robots: Vec<_> = input.lines().map(Robot::from_str).collect();

    for robot in robots.iter_mut() {
        robot.advance(steps, x_size, y_size);
    }

    // let mut robots_string = String::new();

    // for robot in robots.iter() {
    //     writeln!(robots_string, "({},{})", robot.x, robot.y).unwrap();
    // }

    // println!("{}", robots_string);

    let quadrant_counts =
        robots
            .iter()
            .fold(HashMap::<Quadrant, i64>::new(), |mut counts, robot| {
                let quadrant = robot.quadrant(x_size, y_size);
                *counts.entry(quadrant).or_default() += 1;
                counts
            });

    println!("{quadrant_counts:?}");
    quadrant_counts
        .iter()
        .filter(|(q, _)| **q != Quadrant::Axis)
        .map(|(_, count)| count)
        .product()
}

pub fn part2() -> i64 {
    let input = read_input();
    let x_size = 101;
    let y_size = 103;

    let mut robots: Vec<_> = input.lines().map(Robot::from_str).collect();

    let base_grid = vec!['.'; x_size as usize * y_size as usize];
    // the point (i,j) is grid[i + x_size * j]

    for i in 1..10000 {
        let mut cloned_grid = base_grid.clone();
        for robot in robots.iter_mut() {
            robot.advance(1, x_size, y_size);
            cloned_grid[(robot.x + x_size * robot.y) as usize] = 'x';
        }
        println!("{}", i);
        for y in 0..y_size {
            let idx_start = y * x_size;
            let idx_end = (y + 1) * x_size;
            println!(
                "{}",
                cloned_grid[idx_start as usize..idx_end as usize]
                    .iter()
                    .collect::<String>()
            );
        }
    }

    0
}
