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

//! Common utilities for the challenges

use anyhow::Context;
use std::ops::Sub;

pub trait InputProvider {
    /// Returns the input as a string
    ///
    /// # Errors
    /// If the input cannot be read or is not valid an error is returned
    fn get_input(&self, name: &str) -> anyhow::Result<String>;
}

pub struct LazyInputProvider(&'static str);

impl LazyInputProvider {
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        Self(path)
    }
}

#[macro_export]
macro_rules! lazy_input {
    ($day: literal) => {
        LazyInputProvider::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/day",
            $day,
            "/input"
        ))
    };
}

impl InputProvider for LazyInputProvider {
    fn get_input(&self, name: &str) -> anyhow::Result<String> {
        std::fs::read_to_string(std::path::Path::new(self.0).join(name))
            .context("failed to read input file")
    }
}

pub fn abs_diff<T: PartialOrd + Sub>(a: T, b: T) -> T::Output {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub mod nom {
    use nom::combinator::all_consuming;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{space0, u32},
        error::VerboseError,
        multi::fill,
        sequence::{delimited, terminated},
        Finish, IResult, InputLength, Parser,
    };
    use std::fmt::Debug;

    pub type ParseResult<'a, T, I = &'a str> = IResult<I, T, VerboseError<I>>;

    /// Parses a u32 and casts it to a usize
    ///
    /// # Errors
    /// If the input is not a valid u32 an error is returned
    pub fn parse_usize(s: &str) -> nom::IResult<&str, usize, nom::error::VerboseError<&str>> {
        u32.map(|n| n as usize).parse(s)
    }

    /// Parses a sequence of N space delimited u32s as an array of usize.
    /// After the Nth element, the parser completes and returns the remaining with the result, even if it contains more elements.
    ///
    /// Uses `parse_usize` internally.
    ///
    /// # Errors
    /// An error is returned if any of:
    /// - The `parse_usize` fails for any of the elements in the sequence
    /// - The input does not start with N space-delimited u32s
    pub fn parse_usize_array<const N: usize>(
        input: &str,
    ) -> IResult<&str, [usize; N], VerboseError<&str>> {
        let mut data = [0; N];

        let result = fill(
            |input| delimited(space0, parse_usize, space0)(input),
            &mut data[..],
        )(input);

        result.map(move |(rem, _)| (rem, data))
    }

    /// Parses a matrix of N x M space/newline delimited u32s as an array arrays of usize.
    /// After the Mth line, the parser completes and returns the remaining with the result, even if it contains more elements.
    /// Each line must contain exactly N elements.
    ///
    /// Uses `parse_usize_array` internally.
    ///
    /// # Errors
    /// An error is returned if any of:
    /// - The `parse_usize_array` fails for any of the lines in the matrix
    /// - Any line does not contain exactly N elements
    /// - The input contains less than M lines
    pub fn parse_usize_matrix<const N: usize, const M: usize>(
        input: &str,
    ) -> IResult<&str, [[usize; N]; M], VerboseError<&str>> {
        let mut data = [[0; N]; M];

        let result = fill(
            |input| terminated(parse_usize_array, alt((tag("\n"), tag(""))))(input),
            &mut data[..],
        )(input);

        result.map(move |(rem, _)| (rem, data))
    }

    /// Parses an input completely and returns the parsed value.
    ///
    /// # Errors
    /// Any errors that occur during parsing are passed through unchanged.
    /// If the parser completes, but the input is not fully consumed, an error is returned.
    pub fn parse_all<I, O, P>(parser: P, data: I) -> anyhow::Result<O>
    where
        I: InputLength + Debug,
        P: Parser<I, O, VerboseError<I>>,
    {
        all_consuming(parser)(data)
            .finish()
            .map_err(|e| anyhow::anyhow!("{:?}", e))
            .map(|(_, data)| data)
    }
}
