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

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};
use std::ops::AddAssign;

#[derive(Clone)]
pub struct Polymer(Vec<char>);

#[derive(Debug)]
pub struct PairCounters(HashMap<(char, char), usize>);

#[derive(Debug)]
pub struct PolymerizationRules {
    pairs: HashMap<(char, char), char>,
}

impl Polymer {
    pub fn grow(&mut self, rules: &PolymerizationRules) {
        // At most, the polymer will grow by one element between each pair,
        // we pre-allocate the maximum possible size (2n - 1).
        let mut elements = Vec::with_capacity(2 * self.0.len() - 1);

        for pair in self.0.iter().copied().tuple_windows::<(_, _)>() {
            elements.push(pair.0);

            if let Some(extra) = rules.pairs.get(&pair) {
                elements.push(*extra);
            }
        }

        elements.push(*self.0.last().unwrap());

        self.0 = elements;
    }

    pub fn elements(&self) -> &[char] {
        &self.0
    }
}

impl From<&Polymer> for PairCounters {
    fn from(polymer: &Polymer) -> Self {
        Self(
            polymer
                .elements()
                .iter()
                .copied()
                .tuple_windows::<(_, _)>()
                .counts(),
        )
    }
}

impl PairCounters {
    pub fn project_growth(&mut self, rules: &PolymerizationRules) {
        let mut new_counters: HashMap<(char, char), usize> = HashMap::with_capacity(self.0.len());

        for (&(a, b), &count) in self.0.iter() {
            if let Some(&extra) = rules.pairs.get(&(a, b)) {
                new_counters
                    .entry((a, extra))
                    .or_default()
                    .add_assign(count);

                new_counters
                    .entry((extra, b))
                    .or_default()
                    .add_assign(count);
            } else {
                new_counters.entry((a, b)).or_default().add_assign(count);
            }
        }

        self.0 = new_counters;
    }

    /// Returns the number of occurrences of each element in the polymer.
    /// The last element in the polymer will not be counted as it is not the left side of any pair.
    pub fn into_element_counters(self) -> HashMap<char, usize> {
        let mut counters: HashMap<char, usize> = HashMap::new();

        for ((a, _), count) in self.0.into_iter() {
            counters.entry(a).or_default().add_assign(count);
        }

        counters
    }
}

#[derive(Debug)]
pub struct Data {
    pub template: Polymer,
    pub rules: PolymerizationRules,
}

impl Debug for Polymer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Polymer(")?;
        for c in self.0.iter() {
            f.write_char(*c)?;
        }
        f.write_str(")")
    }
}
