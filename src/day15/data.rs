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

use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T = usize> {
    cells: Vec<T>,
    size: usize,
}

impl Grid<usize> {
    pub fn from_input(input: &str) -> Self {
        let mut grid = Grid::new(input.lines().count());

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[(x, y)] = c.to_digit(10).unwrap() as usize;
            }
        }

        grid
    }
}

impl<T> Grid<T> {
    pub fn new(size: usize) -> Self
    where
        T: Default + Copy,
    {
        Grid {
            cells: vec![T::default(); size * size],
            size,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.size() + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.size + x]
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size() {
            for x in 0..self.size() {
                write!(f, "{:>3}", self[(x, y)])?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
