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

use super::*;
use aoc2021::nom::ParseResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, newline};
use nom::combinator::iterator;
use nom::combinator::map;
use nom::sequence::{pair, preceded, separated_pair};

fn parse_polymer(input: &str) -> ParseResult<Polymer> {
    map(alpha1, |c: &str| Polymer(c.chars().collect()))(input)
}

fn parse_polymerization_rules(input: &str) -> ParseResult<PolymerizationRules> {
    let mut parse_iterator = iterator(
        input,
        preceded(
            newline,
            separated_pair(pair(anychar, anychar), tag(" -> "), anychar),
        ),
    );

    let rules = PolymerizationRules {
        pairs: parse_iterator.collect(),
    };

    parse_iterator.finish().map(|(r, _)| (r, rules))
}

pub fn parse_input(input: &str) -> ParseResult<Data> {
    let (rem, (polymer, rules)) =
        separated_pair(parse_polymer, newline, parse_polymerization_rules)(input)?;

    Ok((
        rem,
        Data {
            template: polymer,
            rules,
        },
    ))
}
