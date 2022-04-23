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

use std::fmt::{Display, Formatter, Write};
use std::ops::Add;

pub mod parsing;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pair {
    pub left: Element,
    pub right: Element,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Element {
    Number(i64),
    Pair(Box<Pair>),
}

enum Explosion {
    Safe,
    Handled,
    Left(i64),
    Right(i64),
    Pair(i64, i64),
}

impl Element {
    pub fn magnitude(&self) -> i64 {
        match self {
            &Element::Number(n) => n,
            Element::Pair(box Pair { left, right }) => {
                (3 * left.magnitude()) + (2 * right.magnitude())
            }
        }
    }

    pub fn reduced(mut self) -> Self {
        self.reduce();
        self
    }

    pub fn reduce(&mut self) {
        while self.reduce_one() {}
    }

    pub fn reduce_one(&mut self) -> bool {
        !matches!(self.explode(0), Explosion::Safe) || self.split()
    }

    #[must_use]
    fn explode(&mut self, depth: usize) -> Explosion {
        match self {
            Element::Number(_) => Explosion::Safe,
            Element::Pair(box Pair { left, right }) => {
                if let (Element::Number(left), Element::Number(right)) = (&left, &right) {
                    if depth >= 4 {
                        let explosion = Explosion::Pair(*left, *right);

                        *self = Element::Number(0);

                        return explosion;
                    }

                    return Explosion::Safe;
                }

                match left.explode(depth + 1) {
                    Explosion::Safe => { /* NOOP */ }
                    x @ (Explosion::Handled | Explosion::Left(_)) => return x,
                    x @ Explosion::Right(_) => return right.handle_explosion(x),
                    Explosion::Pair(left_explosion, right_explosion) => {
                        right.handle_explosion(Explosion::Right(right_explosion));

                        return Explosion::Left(left_explosion);
                    }
                };

                match right.explode(depth + 1) {
                    x @ (Explosion::Handled | Explosion::Safe | Explosion::Right(_)) => x,
                    x @ Explosion::Left(_) => left.handle_explosion(x),
                    Explosion::Pair(left_explosion, right_explosion) => {
                        left.handle_explosion(Explosion::Left(left_explosion));

                        Explosion::Right(right_explosion)
                    }
                }
            }
        }
    }

    fn handle_explosion(&mut self, explosion: Explosion) -> Explosion {
        match (self, explosion) {
            (Element::Number(n), Explosion::Left(x) | Explosion::Right(x)) => {
                *n += x;

                Explosion::Handled
            }

            (Element::Pair(box pair), x @ Explosion::Left(_)) => pair.right.handle_explosion(x),
            (Element::Pair(box pair), x @ Explosion::Right(_)) => pair.left.handle_explosion(x),

            _ => unreachable!(),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            &mut Element::Number(n) if n >= 10 => {
                let half = n / 2;

                *self = Element::from(Pair {
                    left: Element::from(half),
                    right: Element::from(n - half),
                });

                true
            }
            Element::Pair(box Pair { left, right }) => left.split() || right.split(),
            _ => false,
        }
    }
}

impl From<Pair> for Element {
    fn from(pair: Pair) -> Self {
        Self::Pair(Box::new(pair))
    }
}

impl From<i64> for Element {
    fn from(number: i64) -> Self {
        Self::Number(number)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(ref n) => Display::fmt(n, f),
            Element::Pair(box Pair {
                ref left,
                ref right,
            }) => {
                f.write_char('[')?;

                Display::fmt(left, f)?;
                f.write_char(',')?;
                Display::fmt(right, f)?;

                f.write_char(']')
            }
        }
    }
}

impl Add for Element {
    type Output = Element;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(Pair {
            left: self,
            right: rhs,
        }))
        .reduced()
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use aoc2021::nom::parse_all;

    use super::parsing::parse_element;
    use super::*;

    #[test]
    fn add_two_numbers() {
        let sum = Element::from(1) + Element::from(2);

        assert_eq!(
            sum,
            Element::from(Pair {
                left: Element::from(1),
                right: Element::from(2),
            })
        );
    }

    #[test]
    fn add_numbers_and_pairs() {
        let number = Element::from(1);
        let pair = Element::from(Pair {
            left: Element::from(2),
            right: Element::from(3),
        });

        assert_eq!(
            number.clone() + pair.clone(),
            Element::from(Pair {
                left: number.clone(),
                right: pair.clone(),
            })
        );

        assert_eq!(
            pair.clone() + number.clone(),
            Element::from(Pair {
                left: pair,
                right: number,
            })
        );
    }

    #[test]
    fn add_with_reducible_result() {
        let left = parse_all(parse_element, "[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let right = parse_all(parse_element, "[1,1]").unwrap();

        let expected_result =
            parse_all(parse_element, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();

        assert_eq!(left + right, expected_result);
    }

    #[test_case("[[1,2],[[3,4],5]]"     => "[[1,2],[[3,4],5]]"  ; "already reduced"          )]
    #[test_case("[[[[[9,8],1],2],3],4]" => "[[[[0,9],2],3],4]"  ; "leftmost explosion"       )]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]" => "[7,[6,[5,[7,0]]]]"  ; "rightmost explosion"      )]
    #[test_case("[[6,[5,[4,[3,2]]]],1]" => "[[6,[5,[7,0]]],3]"  ; "rightmost inner explosion")]
    #[test_case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,3]]]]" => "[[3,[2,[8,0]]],[9,[5,[4,3]]]]"
        ; "bouncing explosion"
    )]
    #[test_case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]" => "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        ; "double explosion"
    )]
    #[test_case(
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]" => "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        ; "multi-stage reduction"
    )]
    fn test_reduce(input: &str) -> String {
        let mut element = parse_all(parse_element, input).unwrap();

        element.reduce();

        element.to_string()
    }

    #[test_case("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]" => "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"    ; "first explosion" )]
    #[test_case("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"     => "[[[[0,7],4],[15,[0,13]]],[1,1]]"       ; "second explosion")]
    #[test_case("[[[[0,7],4],[15,[0,13]]],[1,1]]"       => "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"    ; "first split"     )]
    #[test_case("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"    => "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]" ; "second split"    )]
    #[test_case("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]" => "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"     ; "third explosion" )]
    fn test_reduce_one(input: &str) -> String {
        let mut element = parse_all(parse_element, input).unwrap();

        assert!(element.reduce_one());

        element.to_string()
    }

    #[test_case("[[1,2],[[3,4],5]]"                                     => 143 )]
    #[test_case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"                     => 1384)]
    #[test_case("[[[[1,1],[2,2]],[3,3]],[4,4]]"                         => 445 )]
    #[test_case("[[[[3,0],[5,3]],[4,4]],[5,5]]"                         => 791 )]
    #[test_case("[[[[5,0],[7,4]],[5,5]],[6,6]]"                         => 1137)]
    #[test_case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" => 3488)]
    fn test_magnitude(input: &str) -> i64 {
        parse_all(parse_element, input).unwrap().magnitude()
    }
}
