/*
 * MIT License
 *
 * Copyright (c) 2022 Luiz Ferraz
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

use crate::data::{OperatorType, Packet};
use aoc2021::nom::ParseResult;
use nom::{bits, combinator::map, multi, sequence, InputLength};

enum OperatorLength {
    Bits(usize),
    Packets(usize),
}

type BitInput<'a> = (&'a [u8], usize);

fn parse_literal_value(data: BitInput) -> ParseResult<u64, BitInput> {
    let (remainder, result): (BitInput, Vec<u8>) = multi::many0(sequence::preceded(
        bits::complete::tag(1usize, 1usize),
        bits::complete::take(4usize),
    ))(data)?;

    let (remainder, last_part): (BitInput, u8) = sequence::preceded(
        bits::complete::tag(0usize, 1usize),
        bits::complete::take(4_u8),
    )(remainder)?;

    let mut value = 0_u64;

    for part in result {
        value = (value << 4) | u64::from(part);
    }

    value = (value << 4) | u64::from(last_part);

    Ok((remainder, value))
}

fn parse_operator_length(data: BitInput) -> ParseResult<OperatorLength, BitInput> {
    let (remainder, tag): (BitInput, u8) = bits::complete::take(1usize)(data)?;

    match tag {
        0 => map(bits::complete::take(15usize), OperatorLength::Bits)(remainder),
        1 => map(bits::complete::take(11usize), OperatorLength::Packets)(remainder),
        _ => unreachable!(),
    }
}

fn parse_operator_subpackets(data: BitInput) -> ParseResult<Vec<Packet>, BitInput> {
    let (remainder, length) = parse_operator_length(data)?;

    match length {
        OperatorLength::Bits(length) => {
            if remainder.input_len() < length {
                return Err(nom::Err::Error(nom::error::VerboseError {
                    errors: vec![(
                        remainder,
                        nom::error::VerboseErrorKind::Context("not enough data to create packets"),
                    )],
                }));
            }

            let target_remainder_length = remainder.input_len() - length;

            let mut iterator = nom::combinator::iterator(
                remainder,
                sequence::terminated(
                    parse_packet_inner,
                    nom::combinator::verify(
                        nom::combinator::peek(nom::combinator::rest_len),
                        |&len| len >= target_remainder_length,
                    ),
                ),
            );

            let packets: Vec<Packet> = iterator.collect();

            iterator
                .finish()
                .map(move |(remainder, _)| (remainder, packets))
        }
        OperatorLength::Packets(length) => multi::count(parse_packet_inner, length)(remainder),
    }
}

fn parse_u3_tag(data: BitInput) -> ParseResult<u8, BitInput> {
    bits::complete::take(3usize)(data)
}

fn parse_packet_inner(data: BitInput) -> ParseResult<Packet, BitInput> {
    let (remainder, (version, type_id)) = sequence::pair(parse_u3_tag, parse_u3_tag)(data)?;

    // Literal packet fast-path
    if type_id == 4u8 {
        return map(parse_literal_value, |value| Packet::Literal {
            version,
            value,
        })(remainder);
    }

    nom::combinator::map_res(parse_operator_subpackets, move |packets| {
        OperatorType::try_from(type_id).map(|operator_type| Packet::Operator {
            version,
            operator_type,
            packets,
        })
    })(remainder)
}

pub fn parse_packet(data: &[u8]) -> ParseResult<Packet, &[u8]> {
    bits(parse_packet_inner)(data)
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_packet {
        ($name: ident, $input: literal, $packet: expr) => {
            #[test]
            fn $name() {
                let data = hex::decode($input).unwrap();
                let (_, packet) = parse_packet(&data).unwrap();

                let expected = $packet;

                assert_eq!(packet, expected);
            }
        };
    }

    test_packet!(
        literal,
        "D2FE28",
        Packet::Literal {
            version: 6,
            value: 2021
        }
    );

    test_packet!(
        operator_bit_length_type,
        // Explicit extra data that should be ignores
        "38006F45291200EE00D40C823060",
        Packet::Operator {
            version: 1,
            operator_type: OperatorType::LessThan,
            packets: vec![
                Packet::Literal {
                    version: 6,
                    value: 10,
                },
                Packet::Literal {
                    version: 2,
                    value: 20,
                },
            ],
        }
    );

    test_packet!(
        operator_packet_length_type,
        "EE00D40C823060",
        Packet::Operator {
            version: 7,
            operator_type: OperatorType::Maximum,
            packets: vec![
                Packet::Literal {
                    version: 2,
                    value: 1,
                },
                Packet::Literal {
                    version: 4,
                    value: 2,
                },
                Packet::Literal {
                    version: 1,
                    value: 3,
                },
            ],
        }
    );
}
