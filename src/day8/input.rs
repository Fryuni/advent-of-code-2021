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

use aoc2021::nom::ParseResult;

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
pub struct DisplayState {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl DisplayState {
    pub fn segment_count(&self) -> usize {
        self.a as usize
            + self.b as usize
            + self.c as usize
            + self.d as usize
            + self.e as usize
            + self.f as usize
            + self.g as usize
    }

    pub fn overlap_count(&self, other: &DisplayState) -> usize {
        (self.a && other.a) as usize
            + (self.b && other.b) as usize
            + (self.c && other.c) as usize
            + (self.d && other.d) as usize
            + (self.e && other.e) as usize
            + (self.f && other.f) as usize
            + (self.g && other.g) as usize
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DisplayPatterns([DisplayState; 10]);

impl IntoIterator for DisplayPatterns {
    type Item = DisplayState;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InputEntry {
    pub patterns: DisplayPatterns,
    pub digits: [DisplayState; 4],
}

#[derive(Debug, Clone)]
pub struct InputData {
    entries: Vec<InputEntry>,
}

impl InputData {
    pub fn iter(&self) -> impl Iterator<Item = &InputEntry> {
        self.entries.iter()
    }
}

pub struct InputParser;

impl InputParser {
    fn parse_display(s: &str) -> ParseResult<DisplayState> {
        nom::sequence::preceded(
            nom::character::complete::space0,
            nom::multi::fold_many1(
                nom::character::complete::one_of("abcdefg"),
                DisplayState::default,
                |mut acc, c| {
                    match c {
                        'a' => acc.a = true,
                        'b' => acc.b = true,
                        'c' => acc.c = true,
                        'd' => acc.d = true,
                        'e' => acc.e = true,
                        'f' => acc.f = true,
                        'g' => acc.g = true,
                        _ => unreachable!(),
                    }
                    acc
                },
            ),
        )(s)
    }

    fn parse_digits<const N: usize>(s: &str) -> ParseResult<[DisplayState; N]> {
        let mut digits = [Default::default(); N];

        let result = nom::multi::fill(Self::parse_display, &mut digits)(s);

        result.map(|(r, _)| (r, digits))
    }

    fn parse_patterns(s: &str) -> ParseResult<DisplayPatterns> {
        nom::combinator::map(Self::parse_digits::<10>, DisplayPatterns)(s)
    }

    fn parse_entry(s: &str) -> ParseResult<InputEntry> {
        nom::combinator::map(
            nom::sequence::separated_pair(
                Self::parse_patterns,
                nom::bytes::complete::tag(" | "),
                Self::parse_digits::<4>,
            ),
            |(patterns, digits)| InputEntry { patterns, digits },
        )(s)
    }

    pub fn parse_input(s: &str) -> ParseResult<InputData> {
        nom::combinator::map(
            nom::multi::separated_list0(nom::character::complete::newline, Self::parse_entry),
            |entries| InputData { entries },
        )(s)
    }
}

#[test]
fn parse_display() {
    assert_eq!(
        InputParser::parse_display("abcdefg"),
        Ok((
            "",
            DisplayState {
                a: true,
                b: true,
                c: true,
                d: true,
                e: true,
                f: true,
                g: true,
            }
        ))
    );
    assert_eq!(
        InputParser::parse_display(""),
        Err(nom::Err::Error(nom::error::VerboseError {
            errors: vec![(
                "",
                nom::error::VerboseErrorKind::Nom(nom::error::ErrorKind::Many1)
            )],
        }))
    );
}

#[test]
fn parse_digits() {
    assert_eq!(
        InputParser::parse_digits::<4>("abcdefg abcdefg abcdefg abcdefg"),
        Ok((
            "",
            [
                DisplayState {
                    a: true,
                    b: true,
                    c: true,
                    d: true,
                    e: true,
                    f: true,
                    g: true,
                },
                DisplayState {
                    a: true,
                    b: true,
                    c: true,
                    d: true,
                    e: true,
                    f: true,
                    g: true,
                },
                DisplayState {
                    a: true,
                    b: true,
                    c: true,
                    d: true,
                    e: true,
                    f: true,
                    g: true,
                },
                DisplayState {
                    a: true,
                    b: true,
                    c: true,
                    d: true,
                    e: true,
                    f: true,
                    g: true,
                },
            ]
        ))
    );
}
