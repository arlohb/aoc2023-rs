#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::str::FromStr;

use anyhow::{bail, Context};

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for s in s.split(", ") {
            let (n, colour) = s.split_once(' ').context("Invalid input")?;
            let n: u32 = n.parse()?;

            match colour {
                "red" => red += n,
                "green" => green += n,
                "blue" => blue += n,
                _ => bail!("Invalid input"),
            }
        }

        Ok(Self { red, green, blue })
    }
}

impl Set {
    pub const fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub const fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    infos: Vec<Set>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").context("Invalid input")?;
        let (id_str, s) = s.split_once(": ").context("Invalid input")?;

        let infos = s
            .split("; ")
            .map(Set::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            id: id_str.parse()?,
            infos,
        })
    }
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.infos.iter().all(Set::is_possible)
    }
}

const PART1: bool = false;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    if PART1 {
        let id_sum: u32 = games
            .into_iter()
            .filter(Game::is_possible)
            .map(|game| game.id)
            .sum();

        println!("{id_sum}");
    } else {
        let power_sum: u32 = games
            .into_iter()
            .map(|game| -> anyhow::Result<_> {
                let red = game
                    .infos
                    .iter()
                    .map(|info| info.red)
                    .max()
                    .context("No infos for game")?;
                let green = game
                    .infos
                    .iter()
                    .map(|info| info.green)
                    .max()
                    .context("No infos for game")?;
                let blue = game
                    .infos
                    .iter()
                    .map(|info| info.blue)
                    .max()
                    .context("No infos for game")?;

                let set = Set { red, green, blue };
                let power = set.power();

                Ok(power)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum();

        println!("{power_sum}");
    }

    Ok(())
}
