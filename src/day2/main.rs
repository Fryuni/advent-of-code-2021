//! Binary for solving day 2 of Advent of Code 2021

use anyhow::{Context, Error};
use aoc2021::InputProvider;
use include_dir::*;
use std::str::FromStr;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day2/input");

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_closure = || format!("malformed instruction ({})", s);
        let (name, value) = s.split_once(' ').with_context(err_closure)?;

        let value = value.parse().with_context(err_closure)?;

        match name {
            "forward" => Ok(Self::Forward(value)),
            "down" => Ok(Self::Down(value)),
            "up" => Ok(Self::Up(value)),
            _ => Err(Error::msg(err_closure())),
        }
    }
}

fn challenge_one(input: &[Instruction]) -> anyhow::Result<usize> {
    let (h, v) = input
        .iter()
        .fold((0, 0), |(h, v), instruction| match instruction {
            Instruction::Forward(x) => (h + x, v),
            Instruction::Down(x) => (h, v + x),
            Instruction::Up(x) => (h, v - x),
        });

    Ok(h * v)
}

fn challenge_two(input: &[Instruction]) -> anyhow::Result<usize> {
    let (h, v, _) = input
        .iter()
        .fold((0, 0, 0), |(h, v, aim), instruction| match instruction {
            Instruction::Forward(x) => (h + x, v + (aim * x), aim),
            Instruction::Down(x) => (h, v, aim + x),
            Instruction::Up(x) => (h, v, aim - x),
        });

    Ok(h * v)
}

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?;

    let input: Vec<Instruction> = content.lines().map(str::parse).collect::<Result<_, _>>()?;

    println!(
        "Challenge one ({}): {}",
        name,
        challenge_one(&input).context("challenge one")?
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two(&input).context("challenge two")?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
