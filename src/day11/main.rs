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

//! Binary for solving day 11 of Advent of Code 2021
#![feature(backtrace)]

use anyhow::Context;
use aoc2021::InputProvider;
use include_dir::*;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day11/input");

mod data;

fn challenge_one(input: &data::State) -> anyhow::Result<usize> {
    let mut input = input.clone();

    let mut total_flahes = 0;

    for _ in 0..100 {
        total_flahes += input.advance_state();
    }

    Ok(total_flahes)
}

fn challenge_two(input: &data::State) -> anyhow::Result<usize> {
    let mut input = input.clone();

    for step in 1.. {
        if input.advance_state() == 100 {
            return Ok(step);
        }
    }

    unreachable!("For loop should break with the answer")
}

fn process(name: &str) -> anyhow::Result<()> {
    let content = data::Parser::parse_input(
        INPUT_DIR
            .get_input(&format!("{}.txt", name))
            .context("reading content")?,
    )?;

    println!(
        "Challenge one ({}): {}",
        name,
        challenge_one(&content).context("challenge one")?
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two(&content).context("challenge two")?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    colored::control::set_override(true);
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
