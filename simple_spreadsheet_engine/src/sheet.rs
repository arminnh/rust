use std::fmt;

use crate::cell::Cell;

#[derive(Debug)]
pub struct Sheet {
    pub content: Vec<Vec<Cell>>,
}

#[derive(Debug)]
pub struct ProcessedSheet {
    pub content: Vec<Vec<String>>,
}

impl Sheet {
    /// Creates a Sheet with content (2D array of Cells) from a str.
    pub fn parse_input(input: &str) -> Sheet {
        let rows = input
            .lines()
            .map(|line| line.split(',').map(|col| Cell::from(col)).collect())
            .collect();

        Sheet { content: rows }
    }

    /// Processes/resolves all Cells into a ProcessedSheet ready for displaying.
    pub fn process(&self) -> ProcessedSheet {
        let rows = self
            .content
            .iter()
            .map(|row| row.iter().map(|col| col.process(self)).collect())
            .collect();

        ProcessedSheet { content: rows }
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self
            .content
            .iter()
            .enumerate()
            .map(|(i, row)| format!("{}: {:?}", i, row))
            .collect();

        write!(f, "{}", out.join("\n"))
    }
}

impl fmt::Display for ProcessedSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self
            .content
            .iter()
            .enumerate()
            .map(|(i, row)| row.join(", "))
            .collect();

        write!(f, "{}", out.join("\n"))
    }
}
