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

//! Binary for solving day 2 of Advent of Code 2021

use anyhow::{Context, Error};
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use std::str::FromStr;

static INPUT_DIR: LazyInputProvider = lazy_input!(2);

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_closure = || format!("malformed instruction ({})", s);
        let (name, value) = s.split_once(' ').with_context(err_closure)?;

        let value = value.parse().with_context(err_closure)?;

        match name {
            "forward" => Ok(Self::Forward(value)),
            "down" => Ok(Self::Down(value)),
            "up" => Ok(Self::Up(value)),
            _ => Err(Error::msg(err_closure())),
        }
    }
}

fn challenge_one(input: &[Instruction]) -> usize {
    let (h, v) = input
        .iter()
        .fold((0, 0), |(h, v), instruction| match instruction {
            Instruction::Forward(x) => (h + x, v),
            Instruction::Down(x) => (h, v + x),
            Instruction::Up(x) => (h, v - x),
        });

    h * v
}

fn challenge_two(input: &[Instruction]) -> usize {
    let (h, v, _) = input
        .iter()
        .fold((0, 0, 0), |(h, v, aim), instruction| match instruction {
            Instruction::Forward(x) => (h + x, v + (aim * x), aim),
            Instruction::Down(x) => (h, v, aim + x),
            Instruction::Up(x) => (h, v, aim - x),
        });

    h * v
}

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?;

    let input: Vec<Instruction> = content.lines().map(str::parse).collect::<Result<_, _>>()?;

    println!("Challenge one ({}): {}", name, challenge_one(&input));

    println!("Challenge two ({}): {}", name, challenge_two(&input));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
