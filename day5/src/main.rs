#![warn(clippy::unwrap_used, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use anyhow::Context;
use itertools::Itertools;
use std::{ops::Range, str::FromStr};

pub struct Map {
    pub ranges: Vec<(Range<u64>, i64)>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .map(|line| -> anyhow::Result<_> {
                let (dst_start, line) = line.split_once(' ').context("Invalid input")?;
                let (src_start, length) = line.split_once(' ').context("Invalid input")?;

                let dst_start: u64 = dst_start.parse()?;
                let src_start: u64 = src_start.parse()?;
                let length: u64 = length.parse()?;

                Ok((
                    src_start..src_start + length,
                    dst_start as i64 - src_start as i64,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { ranges })
    }
}

impl Map {
    #[must_use]
    pub fn lookup(&self, input: u64) -> u64 {
        self.ranges
            .iter()
            .find(|(range, _)| range.contains(&input))
            .map_or(input, |(_, offset)| {
                u64::try_from(input as i64 + offset).unwrap_or(0)
            })
    }
}

pub struct Pipeline {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl FromStr for Pipeline {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps = s
            .split("\n\n")
            .skip(1)
            .map(|paragraph| paragraph.lines().skip(1).join("\n"))
            .collect::<Vec<_>>();

        Ok(Self {
            seed_to_soil: Map::from_str(&maps[0])?,
            soil_to_fertilizer: Map::from_str(&maps[1])?,
            fertilizer_to_water: Map::from_str(&maps[2])?,
            water_to_light: Map::from_str(&maps[3])?,
            light_to_temperature: Map::from_str(&maps[4])?,
            temperature_to_humidity: Map::from_str(&maps[5])?,
            humidity_to_location: Map::from_str(&maps[6])?,
        })
    }
}

impl Pipeline {
    #[must_use]
    pub fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.lookup(seed);
        let fertilizer = self.soil_to_fertilizer.lookup(soil);
        let water = self.fertilizer_to_water.lookup(fertilizer);
        let light = self.water_to_light.lookup(water);
        let temperature = self.light_to_temperature.lookup(light);
        let humidity = self.temperature_to_humidity.lookup(temperature);

        self.humidity_to_location.lookup(humidity)
    }
}

const PART1: bool = false;

fn parse_seeds(input: &str) -> anyhow::Result<impl Iterator<Item = u64>> {
    let nums = input
        .lines()
        .next()
        .context("invalid input")?
        .split_once(": ")
        .context("invalid input")?
        .1
        .split(' ')
        .map(u64::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(nums.into_iter())
}

fn parse_seeds_2(seeds: impl Iterator<Item = u64>) -> impl Iterator<Item = u64> {
    // Yes, part 2 takes 59 mins and 50 secs to run
    // No, I don't think I'll optimise it
    seeds
        .tuples()
        .flat_map(|(start, length)| start..start + length)
}

fn calc_lowest_location(
    pipeline: &Pipeline,
    seeds: impl Iterator<Item = u64>,
) -> anyhow::Result<u64> {
    seeds
        .map(|seed| pipeline.seed_to_location(seed))
        .min()
        .context("Invalid input")
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let seeds = parse_seeds(&input)?;

    let pipeline = Pipeline::from_str(&input)?;

    let lowest_location = if PART1 {
        calc_lowest_location(&pipeline, seeds)?
    } else {
        calc_lowest_location(&pipeline, parse_seeds_2(seeds))?
    };

    println!("{lowest_location}");

    Ok(())
}
