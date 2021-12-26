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

pub mod parser;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point(usize, usize);

#[derive(Clone)]
pub struct Grid {
    pub dots: HashSet<Point>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FoldInstruction {
    AlongX(usize),
    AlongY(usize),
}

impl FoldInstruction {
    pub fn apply(&self, grid: &Grid) -> Grid {
        let iterator = grid.dots.iter().copied();

        Grid {
            dots: match *self {
                FoldInstruction::AlongX(fold_line) => iterator
                    .filter_map(|point| match point {
                        Point(x, _) if x == fold_line => None,
                        Point(x, y) if x > fold_line => Some(Point(2 * fold_line - x, y)),
                        _ => Some(point),
                    })
                    .collect(),
                FoldInstruction::AlongY(fold_line) => iterator
                    .filter_map(|point| match point {
                        Point(_, y) if y == fold_line => None,
                        Point(x, y) if y > fold_line => Some(Point(x, 2 * fold_line - y)),
                        _ => Some(point),
                    })
                    .collect(),
            },
        }
    }
}

#[derive(Clone)]
pub struct Data {
    pub grid: Grid,
    pub fold_instructions: Vec<FoldInstruction>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|p| p.0).max().unwrap();
        let max_y = self.dots.iter().map(|p| p.1).max().unwrap();

        // Print a matrix of x by y dots
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&Point(x, y)) {
                    f.write_char('#')?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Data: {{\n  FoldInstructions: {:?},\n  Grid:\n{:>4?}}}\n",
            self.fold_instructions, self.grid,
        )
    }
}
