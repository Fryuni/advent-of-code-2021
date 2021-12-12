use super::*;

#[derive(Debug)]
enum Criteria {
    Oxygen,
    CO2,
}

struct StateInteger<'a>(&'a [State]);

impl<'a, T: AsRef<[State]> + ?Sized> From<&'a T> for StateInteger<'a> {
    fn from(s: &'a T) -> Self {
        Self(s.as_ref())
    }
}

impl From<StateInteger<'_>> for usize {
    fn from(s: StateInteger<'_>) -> Self {
        let mut value = 0;

        for (offset, &state) in s.0.iter().rev().enumerate() {
            if let State::One = state {
                value |= 1 << offset;
            }
        }

        value
    }
}

impl Criteria {
    fn apply(criteria: Self, matrix: &Matrix) -> usize {
        // Shallow clone the outer Vec
        let mut values: Vec<&[State]> = matrix.data.iter().map(AsRef::as_ref).collect_vec();

        for index in 0..matrix.width {
            let (mut zeros, mut ones) = (0, 0);
            for &nums in &values {
                match nums[index] {
                    State::Zero => zeros += 1,
                    State::One => ones += 1,
                }
            }

            let filtered = match criteria {
                Criteria::Oxygen => {
                    if zeros > ones {
                        State::Zero
                    } else {
                        State::One
                    }
                }
                Criteria::CO2 => {
                    if ones < zeros {
                        State::One
                    } else {
                        State::Zero
                    }
                }
            };

            values.retain(|v| v[index] == filtered);

            if values.len() == 1 {
                return StateInteger::from(values[0]).into();
            }
        }

        unreachable!();
    }
}

pub fn challenge_two(input: &Matrix) -> anyhow::Result<usize> {
    Ok({
        let oxygen = Criteria::apply(Criteria::Oxygen, input);

        let carbon_dioxide = Criteria::apply(Criteria::CO2, input);

        dbg!(oxygen) * dbg!(carbon_dioxide)
    })
}
