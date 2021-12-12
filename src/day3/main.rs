//! Binary for solving day 3 of Advent of Code 2021
#![feature(type_alias_impl_trait)]

use anyhow::{bail, Context, Error};
use aoc2021::InputProvider;
use include_dir::*;
use itertools::Itertools;
use std::str::FromStr;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day3/input");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Zero,
    One,
}

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    data: Vec<Vec<State>>,
}

impl FromStr for Matrix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '0' => Ok(State::Zero),
                        '1' => Ok(State::One),
                        _ => bail!("invalid char {}", c),
                    })
                    .try_collect()
            })
            .try_collect()
            .map(|data: Vec<Vec<State>>| Self {
                width: data[0].len(),
                data,
            })
    }
}

mod challenge_one;
mod challenge_two;

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?;

    let input: Matrix = content.parse()?;

    println!(
        "Challenge one ({}): {}",
        name,
        challenge_one::challenge_one(&input).context("challenge one")?
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two::challenge_two(&input).context("challenge two")?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
