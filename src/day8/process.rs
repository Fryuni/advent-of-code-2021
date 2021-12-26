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

use crate::input::*;
use itertools::Itertools;

/// Represent the deduced patterns of the display in order.
/// Meaning that the state at index N represent how the wiring shows the digit N.
#[derive(Default, Debug, Copy, Clone)]
pub struct DisplayConclusion([Option<DisplayState>; 10]);

impl DisplayConclusion {
    fn set_definite_conclusion(&mut self, digit: usize, state: DisplayState) {
        self.0[digit] = Some(state);
    }

    fn check_pattern(&self, state: DisplayState) -> Option<u8> {
        self.0
            .iter()
            .find_position(|&&x| x == Some(state))
            .map(|(i, _)| i as u8)
    }

    fn get_pattern(&self, digit: usize) -> Option<DisplayState> {
        self.0[digit]
    }
}

pub struct EntryProcessor {
    input: DisplayPatterns,
    conclusions: DisplayConclusion,
}

impl EntryProcessor {
    pub fn new(input: DisplayPatterns) -> Self {
        Self {
            input,
            conclusions: Default::default(),
        }
    }

    pub fn process_trivial(&mut self) {
        for pattern in self.input {
            match pattern.segment_count() {
                2 => self.conclusions.set_definite_conclusion(1, pattern),
                3 => self.conclusions.set_definite_conclusion(7, pattern),
                4 => self.conclusions.set_definite_conclusion(4, pattern),
                7 => self.conclusions.set_definite_conclusion(8, pattern),
                _ => {} // Non trivial
            }
        }
    }

    /// Infers the patterns of 0, 6 and 9 from the trivial patterns.
    pub fn first_inference(&mut self) {
        // Get trivial patterns for matching
        let one = self.conclusions.get_pattern(1).unwrap();
        let four = self.conclusions.get_pattern(4).unwrap();

        let solved = self.solve_item(&|state| {
            if state.segment_count() != 6 {
                return None;
            }

            Some(if state.overlap_count(&one) == 1 {
                6
            } else if state.overlap_count(&four) == 3 {
                0
            } else {
                9
            })
        });

        debug_assert_eq!(solved, 3);
    }

    pub fn second_inference(&mut self) {
        // Get trivial patterns for matching
        let one = self.conclusions.get_pattern(1).unwrap();
        let six = self.conclusions.get_pattern(6).unwrap();

        let solved = self.solve_item(&|state| {
            if state.segment_count() != 5 {
                return None;
            }

            Some(if state.overlap_count(&one) == 2 {
                3
            } else if state.overlap_count(&six) == 5 {
                5
            } else {
                2
            })
        });

        debug_assert_eq!(solved, 3);
    }

    fn solve_item(&mut self, cond: &dyn Fn(DisplayState) -> Option<usize>) -> usize {
        let solution_iterator = self
            .input
            .into_iter()
            .filter_map(move |state| cond(state).map(|i| (i, state)));

        let mut solved = 0;
        for (solution, state) in solution_iterator {
            self.conclusions.set_definite_conclusion(solution, state);
            solved += 1;
        }

        solved
    }

    pub fn apply_conclusions<const N: usize>(
        &self,
        display: &[DisplayState; N],
    ) -> [Option<usize>; N] {
        let mut result = [None; N];

        for (i, &digit) in display.iter().enumerate() {
            if let Some(value) = self.conclusions.check_pattern(digit) {
                result[i] = Some(value as usize);
            }
        }

        result
    }
}
