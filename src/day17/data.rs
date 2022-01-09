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

use aoc2021::binary_search_last;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point(pub i64, pub i64);

#[derive(Debug, Copy, Clone)]
pub struct Area {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Area {
    pub fn new(a: Point, b: Point) -> Area {
        Area {
            top_left: Point(a.0.min(b.0), a.1.max(b.1)),
            bottom_right: Point(a.0.max(b.0), a.1.min(b.1)),
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        self.x_range().contains(&p.0) && self.y_range().contains(&p.1)
    }

    #[inline]
    pub fn min_x(&self) -> i64 {
        self.top_left.0
    }

    #[inline]
    pub fn max_x(&self) -> i64 {
        self.bottom_right.0
    }

    #[inline]
    pub fn x_range(&self) -> std::ops::RangeInclusive<i64> {
        self.min_x()..=self.max_x()
    }

    #[inline]
    pub fn min_y(&self) -> i64 {
        self.bottom_right.1
    }

    #[inline]
    pub fn max_y(&self) -> i64 {
        self.top_left.1
    }

    #[inline]
    pub fn y_range(&self) -> std::ops::RangeInclusive<i64> {
        self.min_y()..=self.max_y()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Probe {
    /// The velocity vector is defined by the point the vector touches starting from the origin.
    pub launch_velocity: Point,
}

impl Probe {
    pub fn launch(horizontal_velocity: i64, vertical_velocity: i64) -> Self {
        Self {
            launch_velocity: Point(horizontal_velocity, vertical_velocity),
        }
    }

    pub fn horizontal_velocity(&self) -> i64 {
        self.launch_velocity.0
    }

    pub fn vertical_velocity(&self, time: i64) -> i64 {
        self.launch_velocity.1 - time
    }

    pub fn get_position(self, time: i64) -> Point {
        Point(
            self.horizontal_position_at(time),
            self.vertical_position_at(time),
        )
    }

    /// Calculate the value of the Y coordinate after N steps starting with V vertical speed.
    ///
    /// Considering:
    /// - The Y coordinate of the probe is independent of the X component, so we can ignore it.
    /// - The Y coordinate after N steps starting with speed V can be calculated directly as:
    ///   ```text
    ///   Y1 = V
    ///   Y2 = V + (V - 1)
    ///   Y3 = V + (V - 1) + (V - 2)
    ///   Which can be generalized as: Yn = (((2V + 1) * N) - (N ^ 2)) / 2
    ///   ```
    pub fn vertical_position_at(self, time: i64) -> i64 {
        let n = time;
        let v = self.launch_velocity.1;

        (((2 * v + 1) * n) - (n * n)) / 2
    }

    /// Calculate the maximum Y coordinate reachable by the probe.
    ///
    /// Differentiating the equation of the Y coordinate with respect to time, we get:
    ///  ```text
    ///  Y = V * t - (t ^ 2) / 2
    ///  Y' = V - t
    ///  ```
    ///
    /// The inflection point of the vertical trajectory happens at the zero of its derivative.
    ///  ```text
    ///  Y'(t) = 0 => 0 = V - t => t = V
    ///  ```
    ///
    /// The maximum Y coordinate is the value of the vertical trajectory at the inflection point.
    ///  ```text
    ///  Y(V) = V * V - (V ^ 2) / 2
    ///  Y(V) = (V ^ 2) / 2
    ///  ```
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn vertical_apogee(self) -> i64 {
        let v = self.launch_velocity.1 as f64;

        ((v + 0.5).powi(2) / 2.).round() as i64
    }

    /// Calculate the value of the X coordinate after N steps starting with V horizontal speed.
    ///
    /// Considering:
    /// - The X coordinate of the probe is independent of the Y component, so we can ignore it.
    /// - The X coordinate after N steps starting with speed V can be calculated directly as:
    ///   ```text
    ///   X1 = V
    ///   X2 = V + (V - 1)
    ///   X3 = V + (V - 1) + (V - 2)
    ///   Which can be generalized as: Xn = (((2V + 1) * N) - (N ^ 2)) / 2
    ///   ```
    pub fn horizontal_position_at(self, time: i64) -> i64 {
        let v = self.launch_velocity.0;
        let n = time.signum() * i64::min(time.abs(), v.abs());

        v.signum() * (((2 * v.abs() + 1) * n) - (n * n)) / 2
    }

    pub fn intersects(&self, area: Area) -> Option<Point> {
        binary_search_last(|time| {
            let position = self.get_position(time);

            if area.contains(position) {
                Some(position)
            } else {
                None
            }
        })
        .ok()
        .map(|(_, position)| position)
    }
}
