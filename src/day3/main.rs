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

//! Binary for solving day 3 of Advent of Code 2021
#![feature(type_alias_impl_trait)]

use anyhow::{bail, Context, Error};
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use itertools::Itertools;
use std::str::FromStr;

static INPUT_DIR: LazyInputProvider = lazy_input!(3);

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
        challenge_one::challenge_one(&input)
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two::challenge_two(&input)
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
