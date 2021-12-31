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

use include_dir::{Dir, File};
use std::ops::Sub;

pub trait InputProvider {
    fn get_input(&self, name: &str) -> anyhow::Result<&'static str>;
}

impl InputProvider for Dir<'static> {
    fn get_input(&self, name: &str) -> anyhow::Result<&'static str> {
        self.get_file(name)
            .and_then(File::contents_utf8)
            .ok_or_else(|| anyhow::anyhow!("missing file"))
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
        branch::alt, bytes::complete::tag, character::complete::*, error::VerboseError, multi::*,
        sequence::*, Finish, IResult, InputLength, Parser,
    };
    use std::fmt::Debug;

    pub type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

    pub fn parse_usize(s: &str) -> nom::IResult<&str, usize, nom::error::VerboseError<&str>> {
        u32.map(|n| n as usize).parse(s)
    }

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
