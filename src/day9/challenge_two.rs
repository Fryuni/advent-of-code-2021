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

use itertools::Itertools;
use std::fmt::{Debug, Formatter};

struct Controller {
    input: Vec<Vec<u32>>,
    counter: Vec<Vec<u32>>,
}

#[derive(Debug, Copy, Clone)]
struct Coordinate(usize, usize);

impl Controller {
    fn neighboors_of(&self, point: Coordinate) -> Vec<Coordinate> {
        let mut neighboors = Vec::with_capacity(4);
        let x = point.0;
        let y = point.1;
        if x > 0 {
            neighboors.push(Coordinate(x - 1, y));
        }
        if x < self.input[0].len() - 1 {
            neighboors.push(Coordinate(x + 1, y));
        }
        if y > 0 {
            neighboors.push(Coordinate(x, y - 1));
        }
        if y < self.input.len() - 1 {
            neighboors.push(Coordinate(x, y + 1));
        }
        neighboors
    }

    fn get_value(&self, point: Coordinate) -> u32 {
        let x = point.0;
        let y = point.1;
        self.input[y][x]
    }

    fn get_counter(&self, point: Coordinate) -> u32 {
        let x = point.0;
        let y = point.1;
        self.counter[y][x]
    }

    fn get_coordinates(&self) -> Vec<Coordinate> {
        (0..self.input.len())
            .cartesian_product(0..self.input[0].len())
            .map(|(y, x)| Coordinate(x, y))
            .collect()
    }

    fn increase_counter(&mut self, point: Coordinate, value: u32) {
        let x = point.0;
        let y = point.1;
        self.counter[y][x] += value;
    }

    fn calculate_descent(&mut self) {
        let coordinated_values = self.get_coordinates();

        for n in (0..9).rev() {
            for c in coordinated_values.iter().copied() {
                if self.get_value(c) != n {
                    continue;
                }

                let counter = self.get_counter(c);

                for neighboor in self.neighboors_of(c) {
                    if self.get_value(neighboor) < n {
                        self.increase_counter(neighboor, counter);
                        break;
                    }
                }
            }
        }
    }

    fn find_low_points(&self) -> Vec<Coordinate> {
        self.get_coordinates()
            .into_iter()
            .filter(|&point| {
                self.neighboors_of(point)
                    .into_iter()
                    .all(|neighboor| self.get_value(neighboor) > self.get_value(point))
            })
            .collect()
    }
}

pub fn run(input: &[Vec<u32>]) -> anyhow::Result<u32> {
    let mut controller = Controller {
        input: input.to_owned(),
        counter: vec![vec![1; input[0].len()]; input.len()],
    };

    controller.calculate_descent();

    let mut basin_sizes = controller
        .find_low_points()
        .into_iter()
        .map(|point| controller.get_counter(point))
        .collect_vec();

    basin_sizes.sort_unstable();

    Ok(basin_sizes.into_iter().rev().take(3).product())
}

impl Debug for Controller {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Controller {")?;
        f.write_str("\n  input: ")?;

        // Write input as a matrix
        for row in self.input.iter() {
            f.write_str("\n    ")?;
            for value in row.iter() {
                f.write_str(&format!("{:>4}", value))?;
                f.write_str(" ")?;
            }
            f.write_str("\n    ")?;
        }

        f.write_str("\n  counter: ")?;

        // Write counter as a matrix
        for row in self.counter.iter() {
            f.write_str("\n    ")?;
            for value in row.iter() {
                f.write_str(&format!("{:>4}", value))?;
                f.write_str(" ")?;
            }
            f.write_str("\n    ")?;
        }

        f.write_str("\n}")?;

        Ok(())
    }
}
