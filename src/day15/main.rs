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

//! Binary for solving day 15 of Advent of Code 2021

use anyhow::Context;
use aoc2021::InputProvider;
use aoc2021::{lazy_input, LazyInputProvider};
use data::Grid;

mod data;
mod solution;

static INPUT_DIR: LazyInputProvider = lazy_input!(15);

fn challenge_one(input: &Grid) -> usize {
    solution::calculate_cost(input)
}

fn challenge_two(input: &Grid) -> usize {
    let grid_size = input.size();
    let mut expanded_grid = Grid::new(grid_size * 5);

    for y in 0..expanded_grid.size() {
        for x in 0..expanded_grid.size() {
            let offset = (x / grid_size) + (y / grid_size);

            expanded_grid[(x, y)] = (input[(x % grid_size, y % grid_size)] + offset - 1) % 9 + 1;
        }
    }

    solution::calculate_cost(&expanded_grid)
}

fn process(name: &str) -> anyhow::Result<()> {
    let grid = Grid::from_input(
        INPUT_DIR
            .get_input(&format!("{}.txt", name))
            .context("reading content")?
            .as_str(),
    );

    println!("Challenge one ({}): {}", name, challenge_one(&grid));

    println!("Challenge two ({}): {}", name, challenge_two(&grid));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    colored::control::set_override(true);
    process("sample").context("sample data")?;
    process("input").context("real data")?;

    Ok(())
}
