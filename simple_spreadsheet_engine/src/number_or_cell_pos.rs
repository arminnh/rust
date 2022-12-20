use std::fmt;

use crate::{cell::Cell, cell_pos::CellPos, sheet::Sheet};

#[derive(Debug, Clone, PartialEq)]
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
            NumberOrCellPos::CellPos(pos) => match sheet.content[pos.row - 1][pos.col - 1] {
                Cell::Number(n) => Some(n),
                _ => None,
            },
        }
    }
}

impl TryFrom<&str> for NumberOrCellPos {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if let Ok(number) = input.parse::<f64>() {
            return Ok(NumberOrCellPos::Number(number));
        }

        if let Ok(pos) = CellPos::try_from(input) {
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
            NumberOrCellPos::try_from("1").unwrap(),
            NumberOrCellPos::Number(1.0)
        );
        assert_eq!(
            NumberOrCellPos::try_from("-1").unwrap(),
            NumberOrCellPos::Number(-1.0)
        );
        assert_eq!(
            NumberOrCellPos::try_from("3.141592").unwrap(),
            NumberOrCellPos::Number(3.141592)
        );
        assert_eq!(
            NumberOrCellPos::try_from("A1").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1))
        );
        assert_eq!(
            NumberOrCellPos::try_from("ZA99").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new("ZA99".to_string(), 99, 677))
        );
    }

    #[test]
    fn handles_invalid_input() {
        let err = Err("Invalid Number or Cell position.");
        assert_eq!(NumberOrCellPos::try_from(""), err);
        assert_eq!(NumberOrCellPos::try_from("?"), err);
        assert_eq!(NumberOrCellPos::try_from("=123"), err);
        assert_eq!(NumberOrCellPos::try_from("Z0"), err);
        assert_eq!(NumberOrCellPos::try_from("A1:"), err);
        assert_eq!(NumberOrCellPos::try_from(":A1"), err);
        assert_eq!(NumberOrCellPos::try_from("1A:A1"), err);
        assert_eq!(NumberOrCellPos::try_from("=H8 * Z1"), err);
        assert_eq!(NumberOrCellPos::try_from("=9 - 5.795"), err);
        assert_eq!(NumberOrCellPos::try_from("#ERROR#"), err);
    }
}
