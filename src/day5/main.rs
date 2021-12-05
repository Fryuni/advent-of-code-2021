//! Binary for solving day 5 of Advent of Code 2021

use anyhow::Context;
use aoc2021::InputProvider;
use include_dir::*;
use itertools::Itertools;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day5/input");

fn challenge_one(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn challenge_two(input: &str) -> anyhow::Result<usize> {
    Ok(0)
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
