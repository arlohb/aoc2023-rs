#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

const PART1: bool = false;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let sum: u32 = if PART1 {
        input
            .lines()
            .map(|line| {
                let digits = line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<_>>();

                10 * digits.first().copied().unwrap_or(0) + digits.last().copied().unwrap_or(0)
            })
            .sum()
    } else {
        input
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                let mut digits = vec![];

                while !line.is_empty() {
                    if line.find("one") == Some(0) {
                        digits.push(1);
                    } else if line.find("two") == Some(0) {
                        digits.push(2);
                    } else if line.find("three") == Some(0) {
                        digits.push(3);
                    } else if line.find("four") == Some(0) {
                        digits.push(4);
                    } else if line.find("five") == Some(0) {
                        digits.push(5);
                    } else if line.find("six") == Some(0) {
                        digits.push(6);
                    } else if line.find("seven") == Some(0) {
                        digits.push(7);
                    } else if line.find("eight") == Some(0) {
                        digits.push(8);
                    } else if line.find("nine") == Some(0) {
                        digits.push(9);
                    } else if let Some(d) = line.chars().next().unwrap_or('\0').to_digit(10) {
                        digits.push(d);
                    }

                    line.remove(0);
                }

                10 * digits.first().copied().unwrap_or(0) + digits.last().copied().unwrap_or(0)
            })
            .sum()
    };

    println!("{sum}");

    Ok(())
}
