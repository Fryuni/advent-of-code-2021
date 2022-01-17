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

//! Binary for solving day 17 of Advent of Code 2021
#![allow(dead_code)]

use itertools::Itertools;

mod data;

fn challenge_one(input: data::Area) -> i64 {
    // After trying to solve this challenge with a purely algorithmic approach,
    // and failing to find a solution, I finally gave up and decided to solve it analytically.

    // The vertical trajectory of the probe over time is a parabola.
    // The zeros of this parabola are the the origin (the starting point)
    // and double the value of the velocity.
    // When the velocity of the probe matches the maximum height of the area,
    // the probe will hit the area exactly at the top. Any faster than that and it will
    // miss the area skipping from above it to zero.
    let max_vy = input.max_y().abs() - 1;

    let probe = data::Probe::launch(0, max_vy);

    probe.vertical_apogee()
}

fn challenge_two(input: data::Area) -> usize {
    let max_x = input.max_x();
    let max_y = input.max_y().abs().max(input.min_y().abs());

    (0..=max_x)
        .cartesian_product(-max_y..=max_y)
        .filter_map(|(x, y)| {
            let probe = data::Probe::launch(x, y);

            probe.intersects(input).map(|time| (probe, time))
        })
        .count()
}

fn process(name: &str, target_area: data::Area) {
    println!("Challenge one ({}): {}", name, challenge_one(target_area));

    println!("Challenge two ({}): {}", name, challenge_two(target_area));
}

fn main() {
    process(
        "sample",
        data::Area::new(data::Point(20, -10), data::Point(30, -5)),
    );

    process(
        "input",
        data::Area::new(data::Point(253, -73), data::Point(280, -46)),
    );
}
