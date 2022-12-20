use std::fmt;

use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Sheet {
    pub content: Vec<Vec<Cell>>,
}

impl Sheet {
    /// Creates a Sheet with content (2D array of Cells) from a str.
    pub fn parse_input(input: String) -> Sheet {
        let rows = input
            .lines()
            .map(|line| line.split(',').map(Cell::from).collect())
            .collect();

        Sheet { content: rows }
    }

    /// Processes/resolves all computations to prepare for displaying.
    pub fn process(&self) -> Sheet {
        let mut processed = self.clone();

        for (i, row) in self.content.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let processed_cell = col.process(self, &processed);
                processed.content[i][j] = processed_cell;
            }
        }

        processed
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self
            .content
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
