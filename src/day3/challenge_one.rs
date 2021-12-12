use super::*;

#[derive(Debug)]
struct StateCounter(Vec<usize>, Vec<usize>);

impl StateCounter {
    fn new(size: usize) -> Self {
        Self(vec![0; size], vec![0; size])
    }

    fn add(&mut self, state: State, position: usize) {
        match state {
            State::Zero => self.0[position] += 1,
            State::One => self.1[position] += 1,
        };
    }
}

impl IntoIterator for StateCounter {
    type Item = (usize, usize);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().zip(self.1.into_iter())
    }
}

pub fn challenge_one(input: &Matrix) -> anyhow::Result<usize> {
    let mut counter = StateCounter::new(input.width);

    for v in &input.data {
        for (position, &state) in v.iter().enumerate() {
            counter.add(state, position);
        }
    }

    let pairs = counter.into_iter().collect_vec();

    let (mut gamma, mut epsilon) = (0, 0);

    for (offset, (zeros, ones)) in pairs.into_iter().rev().enumerate() {
        if ones > zeros {
            gamma |= 1 << offset
        } else {
            epsilon |= 1 << offset
        }
    }

    Ok(gamma * epsilon)
}
