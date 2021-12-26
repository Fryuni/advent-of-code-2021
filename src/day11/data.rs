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

use arrayvec::ArrayVec;
use colored::Colorize;
use itertools::Itertools;
use std::fmt::{Debug, Formatter, Write};

#[derive(Clone)]
pub struct State {
    energy: [[usize; 10]; 10],
}

fn iter_matrix() -> impl Iterator<Item = (usize, usize)> {
    (0..10).cartesian_product(0..10)
}

impl State {
    const FLASHED: usize = 20;

    pub fn advance_state(&mut self) -> usize {
        // println!("Initial state:\n{:?}", self);

        // Advance all the energy levels by 1
        for (i, j) in iter_matrix() {
            self.energy[i][j] += 1;
        }

        // println!("Charged state:\n{:?}", self);

        // Flashes every energy level above 9
        for (i, j) in iter_matrix() {
            self.flash(i, j);
        }

        // println!("Flashing state:\n{:?}", self);

        let mut total_flashes = 0;

        // Reset all the energy levels that went above 9
        for (i, j) in iter_matrix() {
            if self.energy[i][j] > 9 {
                self.energy[i][j] = 0;
                total_flashes += 1;
            }
        }

        // println!("Cooldown state:\n{:?}", self);

        total_flashes
    }

    fn flash(&mut self, x: usize, y: usize) {
        let level = self.energy[x][y];

        if !(10..Self::FLASHED).contains(&level) {
            return;
        }

        self.energy[x][y] = Self::FLASHED;

        let neighbours = (x.saturating_sub(1)..=9.min(x + 1))
            .cartesian_product(y.saturating_sub(1)..=9.min(y + 1));

        for (i, j) in neighbours {
            self.energy[i][j] = self.energy[i][j].saturating_add(1);
            self.flash(i, j);
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_input(input: &str) -> anyhow::Result<State> {
        input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<ArrayVec<_, 10>>()
                    .into_inner()
                    .map_err(|_| anyhow::anyhow!("incorrect number of elements on line"))
                    .and_then(|v| {
                        anyhow::ensure!(v.iter().all(|&x| x < 10), "invalid number");

                        Ok(v)
                    })
            })
            .collect::<Result<ArrayVec<_, 10>, _>>()?
            .into_inner()
            .map(|data| State { energy: data })
            .map_err(|_| anyhow::anyhow!("incorrect number of lines"))
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Print data as a matrix
        for row in &self.energy {
            for &col in row {
                write!(
                    f,
                    "{:>2} ",
                    col.to_string().color(match col {
                        0 => colored::Color::BrightWhite,
                        9 => colored::Color::BrightYellow,
                        x if x >= State::FLASHED => colored::Color::BrightMagenta,
                        x if x > 9 => colored::Color::BrightRed,
                        _ => colored::Color::White,
                    })
                )?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
