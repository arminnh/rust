use std::fmt;

use crate::cell::Cell;

#[derive(Debug)]
pub struct Sheet {
    pub content: Vec<Vec<Cell>>,
}

impl Sheet {
    pub fn parse_input(input: &str) -> Sheet {
        let rows = input
            .lines()
            .map(|line| line.split(',').map(|col| Cell::from(col)).collect())
            .collect();

        Sheet { content: rows }
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
