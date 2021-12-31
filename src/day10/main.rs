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

//! Binary for solving day 10 of Advent of Code 2021

use anyhow::Context;
use itertools::Itertools;
use tap::{Pipe, Tap};

use aoc2021::{lazy_input, nom::parse_all, InputProvider, LazyInputProvider};

use crate::input::{Bracket, LineResult};

static INPUT_DIR: LazyInputProvider = lazy_input!(10);

mod input;

fn challenge_one(input: &input::Input) -> usize {
    input
        .lines
        .iter()
        .map(input::Line::validate)
        .filter_map(|result| match result {
            LineResult::Ok | LineResult::Incomplete { .. } => None,
            LineResult::Corrupted { found, .. } => Some(found),
        })
        .map(|bracket| match bracket {
            Bracket::CloseRound => 3,
            Bracket::CloseSquare => 57,
            Bracket::CloseCurly => 1197,
            Bracket::CloseAngle => 25137,
            _ => unreachable!("Opening brackets should not be unexpected"),
        })
        .sum()
}

fn challenge_two(input: &input::Input) -> usize {
    input
        .lines
        .iter()
        .map(input::Line::validate)
        .filter_map(|result| match result {
            LineResult::Ok | LineResult::Corrupted { .. } => None,
            LineResult::Incomplete { missing_brackets } => Some(missing_brackets),
        })
        .map(|missing_brackets| {
            missing_brackets
                .into_iter()
                .rev()
                .fold(0, |acc, bracket| match bracket {
                    Bracket::CloseRound => (acc * 5) + 1,
                    Bracket::CloseSquare => (acc * 5) + 2,
                    Bracket::CloseCurly => (acc * 5) + 3,
                    Bracket::CloseAngle => (acc * 5) + 4,
                    _ => unreachable!("Opening brackets should not be unexpected"),
                })
        })
        .collect_vec()
        .tap_mut(|result| result.sort_unstable())
        .pipe(|result| result[(result.len() - 1) / 2])
}

fn process(name: &str) -> anyhow::Result<()> {
    let data = parse_all(
        input::Parser::parse_input,
        INPUT_DIR
            .get_input(&format!("{}.txt", name))
            .context("reading content")?
            .as_str(),
    )?;

    println!("Challenge one ({}): {}", name, challenge_one(&data));

    println!("Challenge two ({}): {}", name, challenge_two(&data));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
