#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::{collections::HashSet, str::FromStr};

use anyhow::Context;

struct Card {
    pub wins: HashSet<u32>,
    pub nums: HashSet<u32>,
}

impl Card {
    pub fn win_count(&self) -> u32 {
        self.wins.intersection(&self.nums).count() as u32
    }

    pub fn score(&self) -> u64 {
        let win_count = self.win_count();

        if win_count == 0 {
            0
        } else {
            2u64.pow(win_count - 1)
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(": ").context("Invalid input")?;
        let (wins_s, nums_s) = s.split_once(" | ").context("Invalid input")?;

        let wins = wins_s
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<HashSet<_>, _>>()?;
        let nums = nums_s
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Self { wins, nums })
    }
}

fn process_cards(cards: &mut [(Card, u64)]) {
    let cards_len = cards.len();

    for i in 0..cards.len() {
        let (card, this_count) = &mut cards[i];
        let this_count = *this_count;
        let wins = card.win_count();

        for (_, count) in &mut cards[i + 1..=(i + wins as usize).min(cards_len - 1)] {
            *count += this_count;
        }
    }
}

const PART1: bool = false;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let sum: u64 = if PART1 {
        input
            .lines()
            .map(Card::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(Card::score)
            .sum()
    } else {
        let mut cards = input
            .lines()
            .map(Card::from_str)
            .map(|card| card.map(|card| (card, 1)))
            .collect::<Result<Vec<_>, _>>()?;

        process_cards(&mut cards);

        cards.into_iter().map(|(_, count)| count).sum()
    };

    println!("{sum}");

    Ok(())
}
