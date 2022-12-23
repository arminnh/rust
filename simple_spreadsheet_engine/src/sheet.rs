use std::{collections::HashSet, fmt};

use crate::cell::Cell;

#[derive(Debug)]
pub struct Sheet {
    pub cells: Vec<Vec<Cell>>,
    resolved: Vec<Vec<Cell>>,
    resolved_set: HashSet<(usize, usize)>,
}

impl Sheet {
    /// Creates a Sheet with content (2D array of Cells) from a str.
    pub fn parse_input(input: String) -> Sheet {
        let rows: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.split(',')
                    .enumerate()
                    .map(|(j, cell)| Cell::parse(i, j, cell))
                    .collect()
            })
            .collect();
        let resolved = rows.iter().map(|_| Vec::new()).collect();

        Sheet {
            cells: rows,
            resolved: resolved,
            resolved_set: HashSet::new(),
        }
    }

    /// Processes/resolves all computations to prepare for displaying.
    pub fn resolve(&mut self) {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                col.resolve(i, j, self);
            }
        }
    }

    pub fn get_resolved(&self, row: usize, col: usize) -> &Cell {
        &self.resolved[row][col]
    }

    pub fn set_resolved(&mut self, row: usize, col: usize, cell: Cell) {
        self.resolved[row][col] = cell;
        self.resolved_set.insert((row, col));
    }

    pub fn is_resolved(&self, row: usize, col: usize) -> bool {
        self.resolved_set.contains(&(row, col))
    }

    pub(crate) fn render(&self) -> String {
        todo!()
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|col| format!("{}", col))
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect();

        write!(f, "{}", out.join("\n"))
    }
}
