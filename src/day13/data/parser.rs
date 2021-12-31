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

use nom::bytes::complete::tag;
use nom::character::complete::{newline, one_of};
use nom::combinator::map;
use nom::multi::{count, separated_list1};
use nom::sequence::{preceded, separated_pair};

use aoc2021::nom::{parse_usize, ParseResult};

use super::{Data, FoldInstruction, Grid, Point};

fn parse_point(input: &str) -> ParseResult<Point> {
    map(
        separated_pair(parse_usize, tag(","), parse_usize),
        |(x, y)| Point(x, y),
    )(input)
}

fn parse_grid(input: &str) -> ParseResult<Grid> {
    map(separated_list1(newline, parse_point), |points| Grid {
        dots: points.into_iter().collect(),
    })(input)
}

fn parse_instruction(input: &str) -> ParseResult<FoldInstruction> {
    map(
        preceded(
            tag("fold along "),
            separated_pair(one_of("xy"), tag("="), parse_usize),
        ),
        |(axis, value)| match axis {
            'x' => FoldInstruction::AlongX(value),
            'y' => FoldInstruction::AlongY(value),
            _ => unreachable!(),
        },
    )(input)
}

pub fn parse_input(input: &str) -> ParseResult<Data> {
    map(
        separated_pair(
            parse_grid,
            count(newline, 2),
            separated_list1(newline, parse_instruction),
        ),
        |(grid, instructions)| Data {
            grid,
            fold_instructions: instructions,
        },
    )(input)
}
