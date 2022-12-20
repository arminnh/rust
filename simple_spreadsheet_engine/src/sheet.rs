use std::fmt;

use crate::cell::Cell;

#[derive(Debug)]
pub struct Sheet {
    pub cells: Vec<Vec<Cell>>,
}

impl Sheet {
    /// Creates a Sheet with content (2D array of Cells) from a str.
    pub fn parse_input(input: String) -> Sheet {
        let rows = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.split(',')
                    .enumerate()
                    .map(|(j, cell)| Cell::parse(i, j, cell))
                    .collect()
            })
            .collect();

        Sheet { cells: rows }
    }

    /// Processes/resolves all computations to prepare for displaying.
    pub fn resolve(&self) -> Sheet {
        let mut resolved = Sheet {
            cells: self.cells.iter().map(|_| Vec::new()).collect(),
        };

        for row in self.cells.iter() {
            for col in row.iter() {
                col.resolve(self, &mut resolved);
            }
        }

        resolved
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
