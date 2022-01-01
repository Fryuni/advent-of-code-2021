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

//! Binary for solving day 16 of Advent of Code 2021

use anyhow::Context;
use aoc2021::nom::parse_all;
use aoc2021::{lazy_input, InputProvider, LazyInputProvider};
use std::borrow::Borrow;

mod data;
mod parser;

static INPUT_DIR: LazyInputProvider = lazy_input!(16);

fn challenge_one(input: &data::Packet) -> usize {
    input
        .linearize()
        .into_iter()
        .map(|x| x.version() as usize)
        .sum()
}

fn challenge_two(input: &data::Packet) -> u64 {
    input.evaluate()
}

fn process(name: &str) -> anyhow::Result<()> {
    let packet = parse_all(
        parser::parse_packet,
        hex::decode(
            INPUT_DIR
                .get_input(&format!("{}.txt", name))
                .context("reading content")?
                .as_str(),
        )?
        .borrow(),
    )?;

    println!("Challenge one ({}): {}", name, challenge_one(&packet));

    println!("Challenge two ({}): {}", name, challenge_two(&packet));

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process("input").context("real data")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::challenge_one;
    use super::challenge_two;
    use crate::parser::parse_packet;
    use aoc2021::nom::parse_all;

    macro_rules! test_challenge {
        ($name: ident, $func: ident($input: literal) => $value: literal) => {
            #[test]
            fn $name() {
                let data = hex::decode($input).unwrap();
                let packet = parse_all(parse_packet, &data).unwrap();

                let result = $func(&packet);

                assert_eq!(result, $value);
            }
        };
    }

    test_challenge!(
        test_challenge_one_1,
        challenge_one("8A004A801A8002F478") => 16
    );

    test_challenge!(
        test_challenge_one_2,
        challenge_one("620080001611562C8802118E34") => 12
    );

    test_challenge!(
        test_challenge_one_3,
        challenge_one("C0015000016115A2E0802F182340") => 23
    );

    test_challenge!(
        test_challenge_one_4,
        challenge_one("A0016C880162017C3686B18A3D4780") => 31
    );

    test_challenge!(
        test_challenge_two_1,
        challenge_two("C200B40A82") => 3
    );

    test_challenge!(
        test_challenge_two_2,
        challenge_two("04005AC33890") => 54
    );

    test_challenge!(
        test_challenge_two_3,
        challenge_two("880086C3E88112") => 7
    );

    test_challenge!(
        test_challenge_two_4,
        challenge_two("CE00C43D881120") => 9
    );

    test_challenge!(
        test_challenge_two_5,
        challenge_two("D8005AC2A8F0") => 1
    );

    test_challenge!(
        test_challenge_two_6,
        challenge_two("F600BC2D8F") => 0
    );

    test_challenge!(
        test_challenge_two_7,
        challenge_two("9C005AC2F8F0") => 0
    );

    test_challenge!(
        test_challenge_two_8,
        challenge_two("9C0141080250320F1802104A08") => 1
    );
}
