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

//! Binary for solving day 8 of Advent of Code 2021
#![feature(type_alias_impl_trait)]

use anyhow::Context;
use aoc2021::nom::parse_all;
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use arrayvec::ArrayVec;
use itertools::Itertools;

mod input;
mod process;

static INPUT_DIR: LazyInputProvider = lazy_input!(8);

fn challenge_one(input: &input::Data) -> anyhow::Result<usize> {
    input
        .iter()
        .map(|entry| {
            let mut processor = process::EntryProcessor::new(entry.patterns);

            processor.process_trivial();

            processor
                .apply_conclusions(&entry.digits)
                .into_iter()
                .filter(Option::is_some)
                .count()
        })
        .sum1()
        .ok_or(anyhow::anyhow!("No patterns found"))
}

fn challenge_two(input: &input::Data) -> usize {
    input
        .iter()
        .map(|entry| {
            let mut processor = process::EntryProcessor::new(entry.patterns);

            processor.process_trivial();
            processor.first_inference();
            processor.second_inference();

            processor
                .apply_conclusions(&entry.digits)
                .into_iter()
                .collect::<Option<ArrayVec<_, 4>>>()
                .expect("all digits should be decoded")
                .into_iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, digit)| {
                    acc + digit * 10usize.pow(i.try_into().expect("i < 4"))
                })
        })
        .sum()
}

fn process(name: &str) -> anyhow::Result<()> {
    let data = parse_all(
        input::Parser::parse_input,
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

    println!("Challenge two ({}): {}", name, challenge_two(&data));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
