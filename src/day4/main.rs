//! Binary for solving day 4 of Advent of Code 2021
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused)]

use anyhow::{bail, Context, Error};
use aoc2021::{
    nom::{parse_all, parse_usize},
    InputProvider,
};
use include_dir::*;
use itertools::Itertools;
use nom::error::VerboseError;
use nom::{Finish, InputIter, Parser, ToUsize};
use tap::{Tap, TapFallible};

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day4/input");

mod board;

#[derive(Debug, Default)]
struct InputData {
    chosen_numbers: Vec<usize>,
    boards: Vec<board::BingoBoard>,
}

fn input_parser<'a>() -> impl nom::Parser<&'a str, InputData, VerboseError<&'a str>> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::multi::separated_list1(nom::bytes::complete::tag(","), parse_usize),
            nom::multi::count(nom::character::complete::newline, 2),
            nom::multi::separated_list1(nom::character::complete::newline, board::parse_board),
        ),
        |(chosen_numbers, boards)| InputData {
            chosen_numbers,
            boards,
        },
    )
}

fn challenge_one(input: &InputData) -> anyhow::Result<usize> {
    let mut boards = input.boards.iter().cloned().collect_vec();

    for &number in input.chosen_numbers.iter() {
        for board in boards.iter_mut() {
            board.mark_value(number);
            if let Some(score) = board.winning_score() {
                return Ok(score * number);
            }
        }
    }

    bail!("No winning score found")
}

fn challenge_two(input: &InputData) -> anyhow::Result<usize> {
    let mut boards = input.boards.iter().cloned().collect_vec();

    for &number in input.chosen_numbers.iter() {
        boards.iter_mut().for_each(|board| board.mark_value(number));

        if boards.len() == 1 {
            if let Some(score) = boards[0].winning_score() {
                return Ok(score * number);
            }
        }

        boards.retain(|board| !board.is_winner());
    }

    bail!("No winning score found")
}

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?;

    let data = parse_all(input_parser(), content)?;

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
