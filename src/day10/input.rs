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
use std::fmt::{Debug, Formatter, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Round,
    Curly,
    Square,
    Angle,
    CloseRound,
    CloseCurly,
    CloseSquare,
    CloseAngle,
}

impl Bracket {
    fn closing(&self) -> Option<Self> {
        match self {
            Bracket::Round => Some(Bracket::CloseRound),
            Bracket::Curly => Some(Bracket::CloseCurly),
            Bracket::Square => Some(Bracket::CloseSquare),
            Bracket::Angle => Some(Bracket::CloseAngle),
            _ => None,
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Line(Vec<Bracket>);

#[derive(Debug)]
pub struct Input {
    pub lines: Vec<Line>,
}

#[derive(Debug)]
pub enum LineResult {
    Ok,
    Incomplete {
        missing_brackets: Vec<Bracket>,
    },
    Corrupted {
        expected: Bracket,
        found: Bracket,
        position: usize,
    },
}

impl Line {
    pub fn validate(&self) -> LineResult {
        let mut missing_brackets = Vec::with_capacity(self.0.len() / 2);

        for (position, &bracket) in self.0.iter().enumerate() {
            match bracket.closing() {
                Some(closing_bracket) => missing_brackets.push(closing_bracket),
                None => match missing_brackets.pop() {
                    Some(expected_bracket) if bracket != expected_bracket => {
                        return LineResult::Corrupted {
                            expected: expected_bracket,
                            found: bracket,
                            position,
                        }
                    }
                    Some(_) => {} // If it does match, it's fine
                    None => panic!("Unopened chunk with bracket: {:?}", bracket),
                },
            }
        }

        if missing_brackets.is_empty() {
            LineResult::Ok
        } else {
            LineResult::Incomplete { missing_brackets }
        }
    }
}

pub struct Parser;

impl Parser {
    fn parse_line(input: &str) -> ParseResult<Line> {
        nom::combinator::map(
            nom::multi::many0(nom::combinator::map(
                nom::character::complete::one_of("()[]{}<>"),
                |c| match c {
                    '(' => Bracket::Round,
                    ')' => Bracket::CloseRound,
                    '[' => Bracket::Square,
                    ']' => Bracket::CloseSquare,
                    '{' => Bracket::Curly,
                    '}' => Bracket::CloseCurly,
                    '<' => Bracket::Angle,
                    '>' => Bracket::CloseAngle,
                    _ => unreachable!(),
                },
            )),
            Line,
        )(input)
    }

    pub fn parse_input(input: &str) -> ParseResult<Input> {
        nom::combinator::map(
            nom::multi::separated_list0(nom::character::complete::newline, Self::parse_line),
            |lines| Input { lines },
        )(input)
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Line`")?;

        for bracket in &self.0 {
            match bracket {
                Bracket::Round => f.write_str("(")?,
                Bracket::Curly => f.write_str("{")?,
                Bracket::Square => f.write_str("[")?,
                Bracket::Angle => f.write_str("<")?,
                Bracket::CloseRound => f.write_str(")")?,
                Bracket::CloseCurly => f.write_str("}")?,
                Bracket::CloseSquare => f.write_str("]")?,
                Bracket::CloseAngle => f.write_str(">")?,
            };
        }

        f.write_char('`')
    }
}
