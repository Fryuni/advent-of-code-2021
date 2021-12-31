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

//! Binary for solving day 5 of Advent of Code 2021
#![feature(int_abs_diff)]
#![allow(dead_code)]
#![allow(unused)]

use anyhow::Context;
use aoc2021::nom::parse_all;
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use itertools::Itertools;
use nom::Parser;

static INPUT_DIR: LazyInputProvider = lazy_input!(5);

mod input;

fn challenge_one(input: &input::Data) -> usize {
    let mut diagram = input::Diagram::new();

    input
        .lines
        .iter()
        .filter(|line| line.is_cardinal())
        .for_each(|line| diagram.add_line(line));

    diagram.get_intersections().len()
}

fn challenge_two(input: &input::Data) -> usize {
    let mut diagram = input::Diagram::new();

    input.lines.iter().for_each(|line| diagram.add_line(line));

    diagram.get_intersections().len()
}

fn process(name: &str) -> anyhow::Result<()> {
    let data = parse_all(
        input::Parser::input,
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
