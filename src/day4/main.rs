/*
 * MIT License
 *
 * Copyright (c) 2021 Luiz Ferraz
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Binary for solving day 4 of Advent of Code 2021
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused)]

use anyhow::{bail, Context, Error};
use aoc2021::{lazy_input, LazyInputProvider};
use aoc2021::{
    nom::{parse_all, parse_usize},
    InputProvider,
};
use itertools::Itertools;
use nom::error::VerboseError;
use nom::{Finish, InputIter, Parser, ToUsize};
use tap::{Tap, TapFallible};

static INPUT_DIR: LazyInputProvider = lazy_input!(4);

mod board;

#[derive(Debug, Default)]
struct InputData {
    chosen_numbers: Vec<usize>,
    boards: Vec<board::Board>,
}

fn input_parser<'a>() -> impl nom::Parser<&'a str, InputData, VerboseError<&'a str>> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::multi::separated_list1(nom::bytes::complete::tag(","), parse_usize),
            nom::multi::count(nom::character::complete::newline, 2),
            nom::multi::separated_list1(nom::character::complete::newline, board::parse),
        ),
        |(chosen_numbers, boards)| InputData {
            chosen_numbers,
            boards,
        },
    )
}

fn challenge_one(input: &InputData) -> anyhow::Result<usize> {
    let mut boards = input.boards.iter().copied().collect_vec();

    for &number in &input.chosen_numbers {
        for board in &mut boards {
            board.mark_value(number);
            if let Some(score) = board.winning_score() {
                return Ok(score * number);
            }
        }
    }

    bail!("No winning score found")
}

fn challenge_two(input: &InputData) -> anyhow::Result<usize> {
    let mut boards = input.boards.iter().copied().collect_vec();

    for &number in &input.chosen_numbers {
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

    let data = parse_all(input_parser(), content.as_str())?;

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
