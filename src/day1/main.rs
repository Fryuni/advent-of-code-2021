//! Binary for solving day 1 of Advent of Code 2021

use anyhow::Context;
use aoc2021::InputProvider;
use include_dir::*;
use itertools::Itertools;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day1/input");

fn parse_input(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .with_context(|| format!("could not parse line {}: {}", 0, line))
        })
        .collect()
}

fn challenge_one(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input)?
        .into_iter()
        .tuple_windows::<(usize, usize)>()
        .fold(0, |v, (a, b)| if b > a { v + 1 } else { v }))
}

fn challenge_two(input: &str) -> anyhow::Result<usize> {
    Ok(parse_input(input)?
        .into_iter()
        .tuple_windows::<(usize, usize, usize)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(usize, usize)>()
        .fold(0, |v, (a, b)| if b > a { v + 1 } else { v }))
}

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?;

    println!(
        "Challenge one ({}): {}",
        name,
        challenge_one(content).context("challenge one")?
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two(content).context("challenge two")?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
