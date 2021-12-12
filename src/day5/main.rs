//! Binary for solving day 5 of Advent of Code 2021
#![feature(int_abs_diff)]
#![allow(dead_code)]
#![allow(unused)]

use anyhow::Context;
use aoc2021::nom::parse_all;
use aoc2021::InputProvider;
use include_dir::*;
use itertools::Itertools;
use nom::Parser;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day5/input");

mod input;

fn challenge_one(input: &input::InputData) -> anyhow::Result<usize> {
    let mut diagram = input::Diagram::new();

    input
        .lines
        .iter()
        .filter(|line| line.is_cardinal())
        .for_each(|line| diagram.add_line(line));

    Ok(diagram.get_intersections().len())
}

fn challenge_two(input: &input::InputData) -> anyhow::Result<usize> {
    let mut diagram = input::Diagram::new();

    input.lines.iter().for_each(|line| diagram.add_line(line));

    Ok(diagram.get_intersections().len())
}

fn process(name: &str) -> anyhow::Result<()> {
    let data = parse_all(
        input::InputParser::input,
        INPUT_DIR
            .get_input(&format!("{}.txt", name))
            .context("reading content")?,
    )?;

    println!(
        "Challenge one ({}): {}",
        name,
        challenge_one(&data).context("challenge one")?
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two(&data).context("challenge two")?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
