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

//! Binary for solving day 14 of Advent of Code 2021

use anyhow::Context;
use aoc2021::nom::parse_all;
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::AddAssign;

static INPUT_DIR: LazyInputProvider = lazy_input!(14);

mod data;

fn extract_answer_from_counters(counters: HashMap<char, usize>) -> anyhow::Result<usize> {
    counters
        .into_iter()
        .minmax_by(|(_, left), (_, right)| left.cmp(right))
        .into_option()
        .map(|((min_c, min_v), (max_c, max_v))| {
            println!("Min: {}  Max: {}", min_c, max_c);

            max_v - min_v
        })
        .ok_or(anyhow::anyhow!("No minmax found"))
}

fn challenge_one(input: &data::Data) -> anyhow::Result<usize> {
    let mut polymer = input.template.clone();

    for _ in 0..10 {
        polymer.grow(&input.rules);
    }

    extract_answer_from_counters(polymer.elements().iter().copied().counts())
}

fn challenge_two(input: &data::Data) -> anyhow::Result<usize> {
    let mut pair_counters = data::PairCounters::from(&input.template);

    for _ in 0..40 {
        pair_counters.project_growth(&input.rules);
    }

    let mut element_counters = pair_counters.into_element_counters();

    element_counters
        .entry(input.template.elements().last().copied().unwrap())
        .or_default()
        .add_assign(1);

    extract_answer_from_counters(element_counters)
}

fn process(name: &str) -> anyhow::Result<()> {
    let data = parse_all(
        data::parser::parse_input,
        INPUT_DIR
            .get_input(&format!("{}.txt", name))
            .context("reading content")?
            .as_str(),
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
