#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use std::str::FromStr;

struct Sequence {
    pub diffs: Vec<Vec<i64>>,
}

impl Sequence {
    pub fn new(seq: Vec<i64>) -> Self {
        let mut diffs = vec![seq];

        while let Some(last) = diffs.last() {
            if is_zeros(last) {
                break;
            }

            let next = calc_diff(last);
            diffs.push(next);
        }

        Self { diffs }
    }

    pub fn predict(&mut self) -> i64 {
        for i in (0..self.diffs.len()).rev() {
            let below = self
                .diffs
                .get(i + 1)
                .map_or(Some(0), |v| v.last().copied())
                .unwrap_or(0);

            let left = self.diffs[i].last().copied().unwrap_or(0);

            let next = below + left;
            self.diffs[i].push(next);
        }

        self.diffs[0].last().copied().unwrap_or(0)
    }
}

fn is_zeros(vals: &[i64]) -> bool {
    vals.iter().all(|v| *v == 0)
}

fn calc_diff(vals: &[i64]) -> Vec<i64> {
    let mut out = vec![];

    for i in 0..vals.len() - 1 {
        let a = vals[i];
        let b = vals[i + 1];

        out.push(b - a);
    }

    out
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let sum: i64 = input
        .lines()
        .filter_map(|line| -> Option<_> {
            line.split_whitespace()
                .map(i64::from_str)
                .collect::<Result<Vec<i64>, _>>()
                .ok()
        })
        .map(Sequence::new)
        .map(|mut seq| seq.predict())
        .sum();

    println!("{sum}");

    Ok(())
}
