#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Context;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node<'a> {
    id: &'a str,
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<'a> Node<'a> {
    pub const fn new(id: &'a str) -> Self {
        Node { id }
    }

    pub const fn start() -> Self {
        Node { id: "AAA" }
    }

    pub fn is_end(&self) -> bool {
        self.id == "ZZZ"
    }

    pub fn is_start_2(&self) -> bool {
        &self.id[2..=2] == "A"
    }

    pub fn is_end_2(&self) -> bool {
        &self.id[2..=2] == "Z"
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> anyhow::Result<Self> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => anyhow::bail!("Invalid input"),
        }
    }
}

type MapInner<'a> = HashMap<Node<'a>, (Node<'a>, Node<'a>)>;

struct Map<'a> {
    map: MapInner<'a>,
}

impl<'a> Map<'a> {
    pub const fn from_inner(map: MapInner<'a>) -> Self {
        Self { map }
    }

    pub fn lookup(&self, node: Node, dir: Direction) -> Node {
        let (left, right) = self.map[&node];
        match dir {
            Direction::Left => left,
            Direction::Right => right,
        }
    }
}

const PART1: bool = false;

fn get_repeat(
    map: &Map,
    start: Node,
    mut directions: impl Iterator<Item = Direction>,
    direction_count: u64,
) -> u64 {
    let mut path = vec![start];

    let repeat;

    loop {
        let dir = directions.next().expect("No directions");
        let next = map.lookup(*path.last().expect("Path empty"), dir);

        if let Some(i) = path.iter().position(|node| *node == next) {
            repeat = path.len() - i;
            break;
        }

        path.push(next);
    }

    lcm(&[repeat as u64, direction_count]).expect("Invalid input")
}

fn prime_factors(mut n: u64) -> HashMap<u64, u64> {
    let original = n;
    let mut factors = HashMap::new();
    let mut i = 2;

    while i < original / 2 {
        if n % i == 0 {
            n /= i;
            *factors.entry(i).or_default() += 1;
        } else {
            i += 1;
        }
    }

    if factors.is_empty() {
        factors.insert(original, 1);
    }

    factors
}

fn lcm(ns: &[u64]) -> anyhow::Result<u64> {
    ns.iter()
        .copied()
        .map(prime_factors)
        .reduce(|mut acc, map| {
            for (k, v) in map {
                let old = acc.entry(k).or_default();
                *old = v.max(*old);
            }

            acc
        })
        .context("No nodes")?
        .into_iter()
        .flat_map(|(n, e)| vec![n; e as usize])
        .reduce(|acc, n| acc * n)
        .context("No nodes")
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .context("Invalid input")?
        .chars()
        .map(Direction::from_char)
        .collect::<Result<Vec<_>, _>>()?;

    let dir_count = instructions.len();
    let mut directions = instructions.into_iter().cycle();

    let map = Map::from_inner(
        lines
            .skip(1)
            .map(|line| {
                let node = &line[0..3];
                let left = &line[7..10];
                let right = &line[12..15];
                (Node::new(node), (Node::new(left), Node::new(right)))
            })
            .collect::<MapInner>(),
    );

    let mut i: u64 = 0;

    if PART1 {
        let mut node = Node::start();

        while !node.is_end() {
            node = map.lookup(
                node,
                directions
                    .next()
                    .context("Ran out of infinite instruction")?,
            );

            i += 1;
        }
    } else {
        i = lcm(&map
            .map
            .keys()
            .copied()
            .filter(Node::is_start_2)
            .map(|node| get_repeat(&map, node, directions.clone(), dir_count as u64))
            .collect::<Vec<_>>())?;
    }

    println!("{i}");

    Ok(())
}
