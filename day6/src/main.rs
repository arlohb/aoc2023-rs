#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::str::FromStr;

use anyhow::Context;

fn solve_quadratic(a: i64, b: i64, c: i64) -> (f64, f64) {
    let discriminant = ((b.pow(2) - 4 * a * c) as f64).sqrt();
    (
        (-b as f64 + discriminant) / (2. * a as f64),
        (-b as f64 - discriminant) / (2. * a as f64),
    )
}

fn num_of_wins(race_time: u64, dist: u64) -> u64 {
    // dist_travelled = (race_time - btn_time) * btn_time;
    // dist_travelled > dist when -btn_time^2 + race_time*btn_time - dist > 0
    let (sol1, sol2) = solve_quadratic(-1, race_time as i64, -(dist as i64));
    let sol1 = (sol1 + 0.00001).ceil() as u64;
    let sol2 = (sol2 - 0.00001).floor() as u64;

    sol1.abs_diff(sol2) + 1
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
