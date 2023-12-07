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

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
struct Card(u8);

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = match s.chars().next().context("Invalid input")? {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            c => c.to_digit(10).context("Invalid input")? as u8,
        };

        Ok(Self(n))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self([
            s[0..=0].parse().context("Invalid input")?,
            s[1..=1].parse().context("Invalid input")?,
            s[2..=2].parse().context("Invalid input")?,
            s[3..=3].parse().context("Invalid input")?,
            s[4..=4].parse().context("Invalid input")?,
        ]))
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut ord = HandType::calc(*self).cmp(&HandType::calc(*other));
        if ord == std::cmp::Ordering::Equal {
            ord = self.0[0].cmp(&other.0[0]);
            if ord == std::cmp::Ordering::Equal {
                ord = self.0[1].cmp(&other.0[1]);
                if ord == std::cmp::Ordering::Equal {
                    ord = self.0[2].cmp(&other.0[2]);
                    if ord == std::cmp::Ordering::Equal {
                        ord = self.0[3].cmp(&other.0[3]);
                        if ord == std::cmp::Ordering::Equal {
                            ord = self.0[4].cmp(&other.0[4]);
                        }
                    }
                }
            }
        }
        ord
    }
}

type HandFreqSet = [Vec<Card>; 5];

fn hand_freq_set(hand: Hand) -> HandFreqSet {
    let mut map: [u8; 13] = [0; 13];

    for card in hand.0 {
        map[card.0 as usize - 2] += 1;
    }

    let mut set = HandFreqSet::default();

    for (card_i, count) in map.into_iter().enumerate() {
        if count != 0 {
            set[count as usize - 1].push(Card(card_i as u8));
        }
    }

    set
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    pub fn calc(hand: Hand) -> Self {
        let set = hand_freq_set(hand);

        if set[4].len() == 1 {
            Self::FiveKind
        } else if set[3].len() == 1 {
            Self::FourKind
        } else if set[2].len() == 1 && set[1].len() == 1 {
            Self::FullHouse
        } else if set[2].len() == 1 {
            Self::ThreeKind
        } else if set[1].len() == 2 {
            Self::TwoPair
        } else if set[1].len() == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut hands = input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(hand, bid)| Hand::from_str(hand).ok().map(|hand| (hand, bid)))
        .filter_map(|(hand, bid)| u32::from_str(bid).ok().map(|bid| (hand, bid)))
        .collect::<Vec<_>>();

    hands.sort_by_key(|(hand, _)| *hand);

    let winnings: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum();

    println!("{winnings}");

    Ok(())
}
