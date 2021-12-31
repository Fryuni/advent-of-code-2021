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

#[repr(u8)]
enum Segment {
    A = 1 << 6,
    B = 1 << 5,
    C = 1 << 4,
    D = 1 << 3,
    E = 1 << 2,
    F = 1 << 1,
    G = 1 << 0,
}

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
pub struct DisplayState(u8);

impl DisplayState {
    pub fn segment_count(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn overlap_count(self, other: DisplayState) -> usize {
        (self.0 & other.0).count_ones() as usize
    }

    fn turn_on(&mut self, segment: Segment) {
        self.0 |= segment as u8;
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
pub struct Entry {
    pub patterns: DisplayPatterns,
    pub digits: [DisplayState; 4],
}

#[derive(Debug, Clone)]
pub struct Data {
    entries: Vec<Entry>,
}

impl Data {
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }
}

pub struct Parser;

impl Parser {
    fn parse_display(s: &str) -> ParseResult<DisplayState> {
        nom::sequence::preceded(
            nom::character::complete::space0,
            nom::multi::fold_many1(
                nom::character::complete::one_of("abcdefg"),
                DisplayState::default,
                |mut acc, c| {
                    match c {
                        'a' => acc.turn_on(Segment::A),
                        'b' => acc.turn_on(Segment::B),
                        'c' => acc.turn_on(Segment::C),
                        'd' => acc.turn_on(Segment::D),
                        'e' => acc.turn_on(Segment::E),
                        'f' => acc.turn_on(Segment::F),
                        'g' => acc.turn_on(Segment::G),
                        _ => unreachable!(),
                    }
                    acc
                },
            ),
        )(s)
    }

    fn parse_digits<const N: usize>(s: &str) -> ParseResult<[DisplayState; N]> {
        let mut digits = [DisplayState::default(); N];

        let result = nom::multi::fill(Self::parse_display, &mut digits)(s);

        result.map(|(r, _)| (r, digits))
    }

    fn parse_patterns(s: &str) -> ParseResult<DisplayPatterns> {
        nom::combinator::map(Self::parse_digits::<10>, DisplayPatterns)(s)
    }

    fn parse_entry(s: &str) -> ParseResult<Entry> {
        nom::combinator::map(
            nom::sequence::separated_pair(
                Self::parse_patterns,
                nom::bytes::complete::tag(" | "),
                Self::parse_digits::<4>,
            ),
            |(patterns, digits)| Entry { patterns, digits },
        )(s)
    }

    pub fn parse_input(s: &str) -> ParseResult<Data> {
        nom::combinator::map(
            nom::multi::separated_list0(nom::character::complete::newline, Self::parse_entry),
            |entries| Data { entries },
        )(s)
    }
}

#[test]
fn parse_display() {
    assert_eq!(
        Parser::parse_display("abcdefg"),
        Ok(("", DisplayState(0b0111_1111)))
    );
    assert_eq!(
        Parser::parse_display(""),
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
        Parser::parse_digits::<4>("abcdefg abcdefg abcdefg abcdefg"),
        Ok((
            "",
            [
                DisplayState(0b0111_1111),
                DisplayState(0b0111_1111),
                DisplayState(0b0111_1111),
                DisplayState(0b0111_1111),
            ]
        ))
    );
}
