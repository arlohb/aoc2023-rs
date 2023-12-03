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

fn next_to_symbol(
    lines: &[&str],
    n_start: usize,
    n_end: usize,
    line_i: usize,
) -> anyhow::Result<bool> {
    for line in &lines[line_i.saturating_sub(1)..=(line_i + 1).min(lines.len() - 1)] {
        for scan_ci in n_start.saturating_sub(1)..=(n_end + 1).min(line.len() - 1) {
            let c = line.chars().nth(scan_ci).context("Invalid input")?;

            if !c.is_ascii_digit() && c != '.' {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn next_to_gear(
    lines: &[&str],
    n_start: usize,
    n_end: usize,
    line_i: usize,
) -> anyhow::Result<Option<(usize, usize)>> {
    for (scan_li, line) in lines
        .iter()
        .enumerate()
        .take((line_i + 2).min(lines.len()))
        .skip(line_i.saturating_sub(1))
    {
        for scan_ci in n_start.saturating_sub(1)..=(n_end + 1).min(line.len() - 1) {
            let c = line.chars().nth(scan_ci).context("Invalid input")?;

            if c == '*' {
                return Ok(Some((scan_li, scan_ci)));
            }
        }
    }

    Ok(None)
}

const PART1: bool = false;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let lines = input.lines().collect::<Vec<_>>();

    let mut sum: u64 = 0;

    if PART1 {
        for (line_i, line) in lines.iter().enumerate() {
            let mut n_pair: Option<(u32, usize)> = None;

            for (c_i, c) in line.chars().chain(std::iter::once('.')).enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    if let Some((n, _)) = &mut n_pair {
                        *n = 10 * *n + digit;
                    } else {
                        n_pair = Some((digit, c_i));
                    }
                } else if let Some((n, n_start)) = n_pair {
                    if next_to_symbol(&lines, n_start, c_i - 1, line_i)? {
                        sum += u64::from(n);
                    };
                    n_pair = None;
                }
            }
        }
    } else {
        #[derive(Debug)]
        struct Gear {
            pub count: u32,
            pub ratio: u64,
        }

        impl Default for Gear {
            fn default() -> Self {
                Self { count: 0, ratio: 1 }
            }
        }

        let mut gears = HashMap::<(usize, usize), Gear>::new();

        for (line_i, line) in lines.iter().enumerate() {
            let mut n_pair: Option<(u32, usize)> = None;

            for (c_i, c) in line.chars().chain(std::iter::once('.')).enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    if let Some((n, _)) = &mut n_pair {
                        *n = 10 * *n + digit;
                    } else {
                        n_pair = Some((digit, c_i));
                    }
                } else if let Some((n, n_start)) = n_pair {
                    if let Some(pos) = next_to_gear(&lines, n_start, c_i - 1, line_i)? {
                        let gear = gears.entry(pos).or_default();
                        gear.count += 1;
                        gear.ratio *= u64::from(n);
                    };
                    n_pair = None;
                }
            }
        }

        sum = gears
            .into_values()
            .filter_map(
                |Gear { count, ratio }| {
                    if count == 2 {
                        Some(ratio)
                    } else {
                        None
                    }
                },
            )
            .sum();
    }

    println!("{sum}");

    Ok(())
}
