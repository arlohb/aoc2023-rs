#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::collections::HashMap;

use anyhow::Context;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node<'a> {
    id: &'a str,
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

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut lines = input.lines();

    let mut instructions = lines
        .next()
        .context("Invalid input")?
        .chars()
        .map(Direction::from_char)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .cycle();

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

    let mut node = Node::start();
    let mut i = 0;

    while !node.is_end() {
        node = map.lookup(
            node,
            instructions
                .next()
                .context("Ran out of infinite instruction")?,
        );

        i += 1;
    }

    println!("{i}");

    Ok(())
}
