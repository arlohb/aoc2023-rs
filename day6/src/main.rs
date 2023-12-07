#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::str::FromStr;

use anyhow::Context;

const fn dist_travelled(race_time: u64, btn_time: u64) -> u64 {
    let travel_time = race_time - btn_time;
    let speed = btn_time;
    travel_time * speed
}

fn num_of_wins(race_time: u64, dist: u64) -> u64 {
    (0..race_time)
        .filter(|btn_time| dist_travelled(race_time, *btn_time) > dist)
        .count() as u64
}

const PART1: bool = false;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (times_str, dists_str) = input.split_once('\n').context("Invalid input")?;

    if PART1 {
        let times = times_str
            .split_whitespace()
            .skip(1)
            .map(u64::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let dists = dists_str
            .split_whitespace()
            .skip(1)
            .map(u64::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let count = times
            .into_iter()
            .zip(dists)
            .map(|(race_time, dist)| num_of_wins(race_time, dist))
            .reduce(|acc, n| acc * n)
            .context("No wins found")?;

        println!("{count}");
    } else {
        let time = times_str
            .split_once(':')
            .context("Invalid input")?
            .1
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()?;
        let dist = dists_str
            .split_once(':')
            .context("Invalid input")?
            .1
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()?;

        let count = num_of_wins(time, dist);

        println!("{count}");
    };

    Ok(())
}
