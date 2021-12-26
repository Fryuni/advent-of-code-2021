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

//! Binary for solving day 9 of Advent of Code 2021

use anyhow::Context;
use aoc2021::InputProvider;
use include_dir::*;
use itertools::Itertools;

static INPUT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/day9/input");

/// Return an iterator over all the pairs of x, y coordinates of the given matrix
fn get_coordinates(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..x).cartesian_product(0..y)
}

fn challenge_one(input: &[Vec<u32>]) -> anyhow::Result<usize> {
    let max_x = input.len();
    let max_y = input[0].len();

    let mut total_sum = 0;

    for (x, y) in get_coordinates(max_x, max_y) {
        let mut adjacent = Vec::with_capacity(4);

        if x > 0 {
            adjacent.push(input[x - 1][y]);
        }
        if x < max_x - 1 {
            adjacent.push(input[x + 1][y]);
        }
        if y > 0 {
            adjacent.push(input[x][y - 1]);
        }
        if y < max_y - 1 {
            adjacent.push(input[x][y + 1]);
        }

        if adjacent.into_iter().all(|n| n > input[x][y]) {
            total_sum += input[x][y] + 1;
        }
    }

    Ok(total_sum as usize)
}

mod challenge_two;

fn process(name: &str) -> anyhow::Result<()> {
    let content = INPUT_DIR
        .get_input(&format!("{}.txt", name))
        .context("reading content")?
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    println!(
        "Challenge one ({}): {}",
        name,
        challenge_one(&content).context("challenge one")?
    );

    println!(
        "Challenge two ({}): {}",
        name,
        challenge_two::run(&content).context("challenge two")?
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
