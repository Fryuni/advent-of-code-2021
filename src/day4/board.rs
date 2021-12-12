use aoc2021::nom::parse_usize_matrix;
use nom::error::VerboseError;
use nom::Parser;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CellState {
    Clear,
    Marked,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Clear
    }
}

#[derive(Default, Clone, Copy)]
pub struct BingoBoard {
    values: [[usize; 5]; 5],
    state: [[CellState; 5]; 5],
}

pub fn parse_board(input: &str) -> nom::IResult<&str, BingoBoard, VerboseError<&str>> {
    nom::combinator::map(parse_usize_matrix::<5, 5>, |matrix| BingoBoard {
        values: matrix,
        state: Default::default(),
    })(input)
}

trait State {
    type ColumnIterator: Iterator<Item = CellState>;
    type ColumnsIterator: Iterator<Item = Self::ColumnIterator>;

    type RowIterator: Iterator<Item = CellState>;
    type RowsIterator: Iterator<Item = Self::RowIterator>;

    fn iter_columns(&self) -> Self::ColumnsIterator;

    fn iter_rows(&self) -> Self::RowsIterator;
}

impl BingoBoard {
    pub fn mark_value(&mut self, value: usize) {
        if let Some((row, col)) = self.find_value(value) {
            self.state[row][col] = CellState::Marked;
        }
    }

    fn find_value(&self, value: usize) -> Option<(usize, usize)> {
        for (row, row_values) in self.values.iter().enumerate() {
            for (col, &cell_value) in row_values.iter().enumerate() {
                if cell_value == value {
                    return Some((row, col));
                }
            }
        }

        None
    }

    pub fn winning_score(&self) -> Option<usize> {
        if !self.is_winner() {
            None
        } else {
            Some(self.current_score())
        }
    }

    fn current_score(&self) -> usize {
        self.iter_cells()
            .filter_map(|(state, value)| match state {
                CellState::Clear => Some(value),
                CellState::Marked => None,
            })
            .sum()
    }

    pub fn is_winner(&self) -> bool {
        self.iter_columns()
            .any(|mut column| column.all(|cell| cell == CellState::Marked))
            || self
                .iter_rows()
                .any(|mut row| row.all(|cell| cell == CellState::Marked))
    }

    fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = CellState> + '_> + '_ {
        (0..5).map(move |i| self.state[i].iter().copied())
    }

    fn iter_columns(&self) -> impl Iterator<Item = impl Iterator<Item = CellState> + '_> + '_ {
        (0..5).map(move |i| self.state.iter().map(move |row| row[i]))
    }
    fn iter_cells(&self) -> impl Iterator<Item = (CellState, usize)> + '_ {
        self.state
            .iter()
            .flatten()
            .copied()
            .zip(self.values.iter().flatten().copied())
    }
}

#[test]
fn clear_board() {
    let board = BingoBoard::default();
    assert_eq!(board.winning_score(), None);
}

impl Debug for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BingoBoard {\n")?;
        f.write_str("  values: [\n")?;
        for row in &self.values {
            f.write_str("    [")?;
            for value in row {
                write!(f, "{:>3}", value)?;
            }
            f.write_str("],\n")?;
        }
        f.write_str("  ],\n")?;

        f.write_str("  state: [\n")?;
        for row in &self.state {
            f.write_str("    [")?;
            for value in row {
                write!(f, " {:?} ", value)?;
            }
            f.write_str("],\n")?;
        }
        f.write_str("  ],\n")?;

        f.write_str("}")?;

        Ok(())
    }
}
