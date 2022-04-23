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

//! Binary for solving day 18 of Advent of Code 2021

#![feature(box_patterns)]

use crate::data::parsing::parse_many;
use crate::data::Element;
use anyhow::{anyhow, Context};
use aoc2021::nom::parse_all;
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use rayon::prelude::*;

static INPUT_DIR: LazyInputProvider = lazy_input!(18);

mod data;

fn challenge_one(input: &[Element]) -> anyhow::Result<i64> {
    // Get the magnitude of the summation of all the elements
    input
        .iter()
        .cloned()
        .reduce(std::ops::Add::add)
        .ok_or_else(|| anyhow!("No elements provided"))
        .map(|element| element.magnitude())
}

fn challenge_two(input: &[Element]) -> anyhow::Result<i64> {
    // Get the maximum magnitude possible, adding just two of the elements

    input
        .into_par_iter()
        .enumerate()
        .flat_map(|(left_index, left)| {
            input
                .into_par_iter()
                .enumerate()
                .filter_map(move |(right_index, right)| {
                    if left_index == right_index {
                        None
                    } else {
                        Some((left.clone() + right.clone()).magnitude())
                    }
                })
        })
        .max()
        .ok_or_else(|| anyhow!("Could not compute maximum magnitude"))
}

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")
        .and_then(|content| parse_all(parse_many, &content))?;

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
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
