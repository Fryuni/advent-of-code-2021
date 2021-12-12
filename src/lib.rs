//! Common utilities for the challenges

use include_dir::{Dir, File};

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

pub mod nom {
    use nom::combinator::all_consuming;
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::*, error::VerboseError, multi::*,
        sequence::*, Finish, IResult, InputLength, Parser,
    };
    use std::fmt::Debug;

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
