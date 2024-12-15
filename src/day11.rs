#![allow(unused)]
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::Read,
    iter::repeat,
    rc::{Rc, Weak},
};

#[allow(dead_code)]
const EXAMPLE: &str = r#"125 17"#;

fn read_input() -> String {
    let mut r = String::new();
    File::open("inputs/day11.txt")
        .unwrap()
        .read_to_string(&mut r)
        .unwrap();
    r
}

struct Stones {
    stones: Vec<u64>,
}

impl Stones {
    fn new() -> Self {
        Self { stones: vec![] }
    }

    fn add_stones<T>(&mut self, stone_iter: T)
    where
        T: Iterator<Item = u64>,
    {
        self.stones.extend(stone_iter);
    }

    fn blink(&mut self) {
        let mut new_stones = Vec::with_capacity(self.stones.len() * 2);

        for &stone in self.stones.iter() {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                new_stones.push(stone_str[0..stone_str.len() / 2].parse().unwrap());
                new_stones.push(stone_str[stone_str.len() / 2..].parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }

        self.stones = new_stones;
    }

    fn len(&self) -> usize {
        self.stones.len()
    }
}

pub fn part1() -> i64 {
    let input = EXAMPLE;
    let blinks = 7;

    let mut stones = Stones::new();
    stones.add_stones(input.split_ascii_whitespace().map(|t| t.parse().unwrap()));

    for _ in 1..=blinks {
        stones.blink();
    }

    stones.len() as i64
}

pub fn part2() -> i64 {
    let input = read_input();
    let blinks = 75;
    let mut stones = TreeStones::new(
        &input
            .split_ascii_whitespace()
            .map(|t| t.parse().unwrap())
            .collect::<Vec<_>>(),
    );

    for i in 1..=blinks {
        println!("step {i}");
        stones.blink();
        println!("{}", stones.len(i))
    }

    stones.len(blinks)
}

#[derive(Default)]
struct TreeStones {
    roots: Vec<Rc<RefCell<TreeNode>>>,
    value_to_node: HashMap<i64, Rc<RefCell<TreeNode>>>,
    leaves: Vec<Rc<RefCell<TreeNode>>>,
}

struct TreeNode {
    value: i64,
    childen: Vec<Rc<RefCell<TreeNode>>>,
    parents: Vec<Weak<RefCell<TreeNode>>>,
}

impl TreeStones {
    fn new(initial_values: &[i64]) -> Self {
        let mut s: Self = Default::default();

        for &v in initial_values {
            let new_node = Rc::new(RefCell::new(TreeNode {
                value: v,
                childen: vec![],
                parents: vec![],
            }));
            s.roots.push(Rc::clone(&new_node));
            s.value_to_node.insert(v, Rc::clone(&new_node));
            s.leaves.push(Rc::clone(&new_node));
        }

        s
    }

    fn blink(&mut self) {
        let mut next_leaves = vec![];

        for leaf in self.leaves.iter() {
            let n = leaf.borrow().value;
            let mut n_blinked = vec![];

            if n == 0 {
                n_blinked.push(1);
            } else {
                let stone_str = n.to_string();
                if stone_str.len() % 2 == 0 {
                    n_blinked.push(stone_str[0..stone_str.len() / 2].parse().unwrap());
                    n_blinked.push(stone_str[stone_str.len() / 2..].parse().unwrap());
                } else {
                    n_blinked.push(n * 2024);
                }
            }

            for child_value in n_blinked {
                match self.value_to_node.get(&child_value) {
                    Some(node) => {
                        leaf.borrow_mut().childen.push(Rc::clone(node));
                        node.borrow_mut().parents.push(Rc::downgrade(leaf));
                    }
                    None => {
                        let new_child_node = Rc::new(RefCell::new(TreeNode {
                            value: child_value,
                            childen: vec![],
                            parents: vec![Rc::downgrade(leaf)],
                        }));
                        leaf.borrow_mut().childen.push(Rc::clone(&new_child_node));
                        self.value_to_node
                            .insert(child_value, Rc::clone(&new_child_node));
                        next_leaves.push(new_child_node);
                    }
                }
            }
        }
        self.leaves = next_leaves;
    }

    fn len(&self, blinks: usize) -> i64 {
        // starting point: the length from any node for 0 blinks is 1
        // then: from one blink, it's the sum over its childen of that many blinks minus 1
        let mut lengths: HashMap<i64, i64> = HashMap::new();

        for _ in 0..blinks {
            let mut new_lengths = HashMap::new();
            for node in self.value_to_node.values() {
                let l = node
                    .borrow()
                    .childen
                    .iter()
                    .map(|child| lengths.get(&child.borrow().value).unwrap_or(&1))
                    .sum();
                new_lengths.insert(node.borrow().value, l);
            }
            lengths = new_lengths;
        }

        self.roots
            .iter()
            .map(|root| lengths.get(&root.borrow().value).unwrap())
            .sum()
    }

    fn dump_by_value(&self) {
        for node in self.value_to_node.values() {
            let children_values: Vec<_> = node
                .borrow()
                .childen
                .iter()
                .map(|x| x.borrow().value.to_string())
                .collect();
            print!("{}: ", node.borrow().value);
            println!("{}", children_values.join(", "));
        }
    }
}
