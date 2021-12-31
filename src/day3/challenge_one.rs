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

use super::{Itertools, Matrix, State};

#[derive(Debug)]
struct StateCounter(Vec<usize>, Vec<usize>);

impl StateCounter {
    fn new(size: usize) -> Self {
        Self(vec![0; size], vec![0; size])
    }

    fn add(&mut self, state: State, position: usize) {
        match state {
            State::Zero => self.0[position] += 1,
            State::One => self.1[position] += 1,
        };
    }
}

impl IntoIterator for StateCounter {
    type Item = (usize, usize);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().zip(self.1.into_iter())
    }
}

pub fn challenge_one(input: &Matrix) -> usize {
    let mut counter = StateCounter::new(input.width);

    for v in &input.data {
        for (position, &state) in v.iter().enumerate() {
            counter.add(state, position);
        }
    }

    let pairs = counter.into_iter().collect_vec();

    let (mut gamma, mut epsilon) = (0, 0);

    for (offset, (zeros, ones)) in pairs.into_iter().rev().enumerate() {
        if ones > zeros {
            gamma |= 1 << offset;
        } else {
            epsilon |= 1 << offset;
        }
    }

    gamma * epsilon
}
