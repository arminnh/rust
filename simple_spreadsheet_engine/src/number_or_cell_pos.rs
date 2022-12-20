use std::fmt;

use crate::{cell::Cell, cell_pos::CellPos, sheet::Sheet};

#[derive(Debug,  PartialEq)]
pub enum NumberOrCellPos {
    // TODO: support generic number types -- https://crates.io/crates/num
    Number(f64),
    CellPos(CellPos),
}

impl NumberOrCellPos {
    /// Return the held literal or the value of the cell at CellPos (this only works if that cell contains a number).
    /// TODO: resolve values through a dependency graph to ensure all references can resolve successfully?
    pub fn resolve(&self, sheet: &Sheet) -> Option<f64> {
        match self {
            NumberOrCellPos::Number(n) => Some(*n),
            NumberOrCellPos::CellPos(pos) => match sheet.cells[pos.row - 1][pos.col - 1] {
                Cell::Number(n) => Some(n),
                _ => None,
            },
        }
    }

    pub fn parse(input: &str) -> Result<Self, &'static str> {
        if let Ok(number) = input.parse::<f64>() {
            return Ok(NumberOrCellPos::Number(number));
        }

        if let Ok(pos) = CellPos::parse(input) {
            return Ok(NumberOrCellPos::CellPos(pos));
        }

        Err("Invalid Number or Cell position.")
    }
}

impl fmt::Display for NumberOrCellPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberOrCellPos::Number(num) => write!(f, "{}", num),
            NumberOrCellPos::CellPos(pos) => write!(f, "{}", pos.str),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cell_pos::CellPos, number_or_cell_pos::NumberOrCellPos};

    #[test]
    fn can_parse_cell_range() {
        assert_eq!(
            NumberOrCellPos::parse("1").unwrap(),
            NumberOrCellPos::Number(1.0)
        );
        assert_eq!(
            NumberOrCellPos::parse("-1").unwrap(),
            NumberOrCellPos::Number(-1.0)
        );
        assert_eq!(
            NumberOrCellPos::parse("3.141592").unwrap(),
            NumberOrCellPos::Number(3.141592)
        );
        assert_eq!(
            NumberOrCellPos::parse("A1").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1))
        );
        assert_eq!(
            NumberOrCellPos::parse("ZA99").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new("ZA99".to_string(), 99, 677))
        );
    }

    #[test]
    fn handles_invalid_input() {
        let err = Err("Invalid Number or Cell position.");
        assert_eq!(NumberOrCellPos::parse(""), err);
        assert_eq!(NumberOrCellPos::parse("?"), err);
        assert_eq!(NumberOrCellPos::parse("=123"), err);
        assert_eq!(NumberOrCellPos::parse("Z0"), err);
        assert_eq!(NumberOrCellPos::parse("A1:"), err);
        assert_eq!(NumberOrCellPos::parse(":A1"), err);
        assert_eq!(NumberOrCellPos::parse("1A:A1"), err);
        assert_eq!(NumberOrCellPos::parse("=H8 * Z1"), err);
        assert_eq!(NumberOrCellPos::parse("=9 - 5.795"), err);
        assert_eq!(NumberOrCellPos::parse("#ERROR#"), err);
    }
}
