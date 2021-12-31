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

//! Binary for solving day 7 of Advent of Code 2021

use anyhow::Context;
use itertools::Itertools;

use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};

static INPUT_DIR: LazyInputProvider = lazy_input!(7);

struct PositionCounters(Vec<usize>);

impl PositionCounters {
    fn from_input(input: &[usize]) -> Self {
        let max = input.iter().max().unwrap();
        let mut counters = vec![0; *max + 1];

        for i in input {
            counters[*i] += 1;
        }

        PositionCounters(counters)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().copied()
    }
}

fn challenge_one(input: &[usize]) -> usize {
    let positions = PositionCounters::from_input(input);

    let mut costs = vec![0; positions.len()];

    for (i, count) in positions.iter().enumerate() {
        for (j, slot) in costs.iter_mut().enumerate() {
            *slot += if i < j {
                count * (j - i)
            } else {
                count * (i - j)
            };
        }
    }

    *costs.iter().min().unwrap()
}

fn challenge_two(input: &[usize]) -> usize {
    let positions = PositionCounters::from_input(input);

    let mut costs = vec![0; positions.len()];

    for (i, count) in positions.iter().enumerate() {
        for (j, slot) in costs.iter_mut().enumerate() {
            let distance = if i < j { j - i } else { i - j };

            *slot += count * (distance + 1) * distance / 2;
        }
    }

    *costs.iter().min().unwrap()
}

fn process(name: &str) -> anyhow::Result<()> {
    let content: Vec<usize> = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?
        .split(',')
        .map(str::parse)
        .try_collect()?;

    println!("Challenge one ({}): {}", name, challenge_one(&content));

    println!("Challenge two ({}): {}", name, challenge_two(&content));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
