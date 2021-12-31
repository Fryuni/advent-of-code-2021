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

use crate::Grid;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Copy, Clone, Eq)]
struct Cost {
    own: usize,
    minimum: usize,
}

impl Cost {
    fn total(&self) -> usize {
        self.own + self.minimum
    }

    fn apply_neighbor(&mut self, cost: usize) -> bool {
        if self.minimum > cost {
            self.minimum = cost;
            true
        } else {
            false
        }
    }
}

impl Grid<Cost> {
    fn set_cost(&mut self, x: usize, y: usize, cost: usize) {
        self[(x, y)].own = cost;
    }

    fn propagate(&mut self) {
        let max_size = self.size();

        let mut queue = VecDeque::new();

        queue.push_back((max_size - 1, max_size - 1));

        while let Some((x, y)) = queue.pop_front() {
            let current_cost = self[(x, y)].total();

            for (nx, ny) in [
                (x, y.wrapping_sub(1)),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x + 1, y),
            ] {
                // Check bounds
                if nx >= max_size || ny >= max_size {
                    continue;
                }

                let neighbor = &mut self[(nx, ny)];

                if neighbor.apply_neighbor(current_cost) {
                    queue.push_back((nx, ny));
                }
            }
        }
    }
}

pub fn calculate_cost(input: &Grid) -> usize {
    let grid_size = input.size();
    let mut cost_grid: Grid<Cost> = Grid::new(input.size());

    for x in (0..grid_size).rev() {
        for y in (0..grid_size).rev() {
            cost_grid.set_cost(x, y, input[(x, y)]);
        }
    }

    cost_grid[(grid_size - 1, grid_size - 1)].minimum = 0;

    cost_grid.propagate();

    cost_grid[(0, 0)].minimum
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total().cmp(&other.total()))
    }
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            own: 0,
            minimum: usize::MAX,
        }
    }
}
