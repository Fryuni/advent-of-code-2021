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

use aoc2021::nom::{parse_usize, ParseResult};
use itertools::{EitherOrBoth, Itertools};
use nom::error::VerboseError;
use nom::{IResult, InputIter, InputLength, Parser};
use std::fmt::{Debug, Display, Formatter, Write};

pub struct InputData {
    pub lines: Vec<Line>,
}

pub struct InputParser;

impl InputParser {
    fn point(input: &str) -> ParseResult<Point> {
        nom::combinator::map(
            nom::sequence::separated_pair(
                parse_usize,
                nom::character::complete::char(','),
                parse_usize,
            ),
            |(x, y)| Point(x, y),
        )(input)
    }

    fn line(input: &str) -> ParseResult<Line> {
        nom::combinator::map(
            nom::sequence::separated_pair(
                Self::point,
                nom::bytes::complete::tag(" -> "),
                Self::point,
            ),
            |(start, end)| Line(start, end),
        )(input)
    }

    pub fn input(input: &str) -> ParseResult<InputData> {
        nom::combinator::map(
            nom::multi::separated_list1(nom::character::complete::newline, Self::line),
            |lines| InputData { lines },
        )(input)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point(usize, usize);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Line(Point, Point);

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.0 .1 == self.1 .1
    }

    pub fn is_vertical(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    pub fn is_cardinal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_diagonal(&self) -> bool {
        !self.is_cardinal()
    }

    pub fn points(&self) -> Vec<Point> {
        let Point(x0, y0) = self.0;
        let Point(x1, y1) = self.1;

        let x_range = if x0 < x1 {
            (x0..=x1).collect_vec()
        } else {
            (x1..=x0).rev().collect_vec()
        };

        let y_range = if y0 < y1 {
            (y0..=y1).collect_vec()
        } else {
            (y1..=y0).rev().collect_vec()
        };

        x_range
            .into_iter()
            .zip_longest(y_range)
            .map(|v| match v {
                EitherOrBoth::Both(x, y) => Point(x, y),
                EitherOrBoth::Left(x) => Point(x, y0),
                EitherOrBoth::Right(y) => Point(x0, y),
            })
            .collect()
    }
}

#[test]
fn line_points_vertical() {
    let line = Line(Point(1, 1), Point(1, 5));
    assert_eq!(
        line.points(),
        vec![
            Point(1, 1),
            Point(1, 2),
            Point(1, 3),
            Point(1, 4),
            Point(1, 5),
        ]
    );
}

#[test]
fn line_points_horizontal() {
    let line = Line(Point(1, 1), Point(5, 1));
    assert_eq!(
        line.points(),
        vec![
            Point(1, 1),
            Point(2, 1),
            Point(3, 1),
            Point(4, 1),
            Point(5, 1),
        ]
    );
}

#[test]
fn line_points_diagonal_plus_plus() {
    let line = Line(Point(1, 1), Point(5, 5));
    assert_eq!(
        line.points(),
        vec![
            Point(1, 1),
            Point(2, 2),
            Point(3, 3),
            Point(4, 4),
            Point(5, 5),
        ]
    );
}

#[test]
fn line_points_diagonal_plus_minus() {
    let line = Line(Point(1, 5), Point(5, 1));
    assert_eq!(
        line.points(),
        vec![
            Point(1, 5),
            Point(2, 4),
            Point(3, 3),
            Point(4, 2),
            Point(5, 1),
        ]
    );
}

#[test]
fn line_points_diagonal_minus_plus() {
    let line = Line(Point(5, 1), Point(1, 5));
    assert_eq!(
        line.points(),
        vec![
            Point(5, 1),
            Point(4, 2),
            Point(3, 3),
            Point(2, 4),
            Point(1, 5),
        ]
    );
}

#[test]
fn line_points_diagonal_minus_minus() {
    let line = Line(Point(5, 5), Point(1, 1));
    assert_eq!(
        line.points(),
        vec![
            Point(5, 5),
            Point(4, 4),
            Point(3, 3),
            Point(2, 2),
            Point(1, 1),
        ]
    );
}

pub struct Diagram {
    pub points: Vec<Point>,
}

impl Diagram {
    pub fn new() -> Diagram {
        Diagram { points: Vec::new() }
    }

    pub fn add_line(&mut self, line: &Line) {
        self.points.extend(line.points());
    }

    pub fn get_intersections(&self) -> Vec<Point> {
        let mut counters = std::collections::HashMap::new();

        self.points.iter().for_each(|point| {
            *counters.entry(*point).or_insert(0) += 1;
        });

        counters
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(point, _)| point)
            .collect()
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.points.iter().map(|p| p.0).max().unwrap();
        let max_y = self.points.iter().map(|p| p.1).max().unwrap();
        let mut counters = vec![vec![0; max_x + 1]; max_y + 1];

        for point in &self.points {
            counters[point.1][point.0] += 1;
        }

        for y in 0..=max_y {
            for x in 0..=max_x {
                if counters[y][x] > 0 {
                    write!(f, "{}", counters[y][x])?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[test]
fn drawing_lines() {
    let mut diagram = Diagram::new();
    diagram.add_line(&Line(Point(1, 1), Point(1, 5)));
    diagram.add_line(&Line(Point(5, 1), Point(5, 3)));
    diagram.add_line(&Line(Point(0, 5), Point(2, 5)));

    assert_eq!(
        format!("{}", diagram),
        "......\n\
         .1...1\n\
         .1...1\n\
         .1...1\n\
         .1....\n\
         121...\n"
    );
}

#[test]
fn drawing_sample_lines() {
    let mut diagram = Diagram::new();

    diagram.add_line(&Line(Point(0, 9), Point(5, 9)));
    diagram.add_line(&Line(Point(0, 9), Point(2, 9)));

    diagram.add_line(&Line(Point(9, 4), Point(3, 4)));
    diagram.add_line(&Line(Point(2, 2), Point(2, 1)));
    diagram.add_line(&Line(Point(7, 0), Point(7, 4)));
    diagram.add_line(&Line(Point(3, 4), Point(1, 4)));

    assert_eq!(
        format!("{}", diagram),
        ".......1..\n\
         ..1....1..\n\
         ..1....1..\n\
         .......1..\n\
         .112111211\n\
         ..........\n\
         ..........\n\
         ..........\n\
         ..........\n\
         222111....\n"
    );
}
