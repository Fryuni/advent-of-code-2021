/*
 * MIT License
 *
 * Copyright (c) 2022 Luiz Ferraz
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

use crate::data::{Area, Probe};
use aoc2021::binary_search_last;

fn reaches_line(height: i64, v: i64) -> bool {
    let discriminant = ((4 * v * (v + 1)) + 1 - (8 * height)) / 4;

    discriminant >= 0
}

pub fn run(area: Area) -> i64 {
    // The goal of this challenge is to find the maximum Y value that can be reached while still
    // passing through the target area.

    let (yv, xv) = binary_search_last(|yv| {
        if !reaches_line(area.bottom_right.1, yv) {
            return None;
        }

        // println!("Testing Yv = {}", yv);
        binary_search_last(|xv| {
            if !reaches_line(area.top_left.0, xv) {
                return None;
            }

            // println!("Testing Xv = {}", xv);

            let probe = Probe::launch(xv, yv);

            probe.intersects(area)
        })
        .ok()
        .map(|(xv, _)| xv)
    })
    .expect("No solution found");

    Probe::launch(xv, dbg!(yv)).vertical_apogee()
}
