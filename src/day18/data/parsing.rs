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

use nom::Parser;

use aoc2021::nom::ParseResult;

use super::{Element, Pair};

pub fn parse_element(input: &str) -> ParseResult<Element> {
    nom::sequence::delimited(
        nom::character::complete::space0,
        nom::character::complete::i64
            .map(Element::Number)
            .or(parse_pair.map(Box::new).map(Element::Pair)),
        nom::character::complete::space0,
    )(input)
}

pub fn parse_pair(input: &str) -> ParseResult<Pair> {
    nom::combinator::map(
        nom::sequence::delimited(
            nom::character::complete::char('['),
            nom::sequence::separated_pair(
                parse_element,
                nom::character::complete::char(','),
                parse_element,
            ),
            nom::character::complete::char(']'),
        ),
        |(left, right)| Pair { left, right },
    )(input)
}

pub fn parse_many(input: &str) -> ParseResult<Vec<Element>> {
    nom::multi::separated_list1(nom::character::complete::line_ending, parse_element)(input)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use aoc2021::nom::parse_all;

    use super::*;

    #[test]
    fn test_parse_element() {
        assert_eq!(parse_element("1"), Ok(("", Element::Number(1))));
        assert_eq!(
            parse_element("[1,2]"),
            Ok((
                "",
                Element::Pair(Box::new(Pair {
                    left: Element::Number(1),
                    right: Element::Number(2)
                }))
            ))
        );
        assert_eq!(
            parse_element("[1,[2, 3]] 4"),
            Ok((
                "4",
                Element::from(Pair {
                    left: Element::Number(1),
                    right: Element::from(Pair {
                        left: Element::Number(2),
                        right: Element::Number(3)
                    })
                })
            ))
        );
    }

    #[test_case("[1,2]"                                                        ; "sample 1")]
    #[test_case("[[1,2],3]"                                                    ; "sample 2")]
    #[test_case("[9,[8,7]]"                                                    ; "sample 3")]
    #[test_case("[[1,9],[8,5]]"                                                ; "sample 4")]
    #[test_case("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"                            ; "sample 5")]
    #[test_case("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]"                    ; "sample 6")]
    #[test_case("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"; "sample 7")]
    fn parsing_examples(input: &str) {
        let element = parse_all(parse_element, input).expect("should parse");

        let element_string = format!("{}", element);

        assert_eq!(input, element_string.as_str());
    }
}
