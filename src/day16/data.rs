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

use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq)]
pub enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        operator_type: OperatorType,
        packets: Vec<Packet>,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OperatorType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl Packet {
    pub fn version(&self) -> u8 {
        match self {
            Packet::Literal { version, .. } | Packet::Operator { version, .. } => *version,
        }
    }

    pub fn linearize(&self) -> Vec<&Packet> {
        let mut result = vec![self];

        if let Packet::Operator { packets, .. } = self {
            for packet in packets {
                result.extend(packet.linearize());
            }
        }

        result
    }

    pub fn evaluate(&self) -> u64 {
        match *self {
            Packet::Literal { value, .. } => value,
            Packet::Operator {
                operator_type,
                ref packets,
                ..
            } => operator_type.apply(packets),
        }
    }
}

impl OperatorType {
    fn apply(self, packets: &[Packet]) -> u64 {
        match self {
            OperatorType::Sum => packets.iter().map(Packet::evaluate).sum(),
            OperatorType::Product => packets.iter().map(Packet::evaluate).product(),
            OperatorType::Minimum => packets
                .iter()
                .map(Packet::evaluate)
                .min()
                .expect("there must be at least one packet"),
            OperatorType::Maximum => packets
                .iter()
                .map(Packet::evaluate)
                .max()
                .expect("there must be at least one packet"),
            OperatorType::GreaterThan => {
                assert_eq!(packets.len(), 2);
                let a = packets[0].evaluate();
                let b = packets[1].evaluate();

                u64::from(a > b)
            }
            OperatorType::LessThan => {
                assert_eq!(packets.len(), 2);
                let a = packets[0].evaluate();
                let b = packets[1].evaluate();

                u64::from(a < b)
            }
            OperatorType::EqualTo => {
                assert_eq!(packets.len(), 2);
                let a = packets[0].evaluate();
                let b = packets[1].evaluate();

                u64::from(a == b)
            }
        }
    }
}
