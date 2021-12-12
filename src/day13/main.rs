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

//! Binary for solving day 13 of Advent of Code 2021

use anyhow::Context;
use aoc2021::InputProvider;
use include_dir::*;
use itertools::Itertools;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day13/input");

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
