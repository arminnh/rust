use crate::cell_pos::CellPos;

#[derive(Debug, PartialEq)]
pub struct CellRange {
    start_cell: CellPos,
    end_cell: CellPos,
}

impl CellRange {
    pub fn from(input: &str) -> Option<Self> {
        // TODO: don't allow invalid ranges
        // TODO: support absolute reference with $
        match input.split(':').collect::<Vec<&str>>()[..] {
            [lhs, rhs] => match (CellPos::from(lhs), CellPos::from(rhs)) {
                (Some(start_cell), Some(end_cell)) => Some(CellRange {
                    start_cell,
                    end_cell,
                }),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn new(start_row: u32, start_col: u32, end_row: u32, end_col: u32) -> Self {
        CellRange {
            start_cell: CellPos::new(start_row, start_col),
            end_cell: CellPos::new(end_row, end_col),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_range::CellRange;

    // TODO: parametrized tests instead of asserting each variant manually.
    // Could do it through macros or with a package https://crates.io/crates/rstest

    #[test]
    fn can_parse_cell_range() {
        assert_eq!(
            CellRange::from("A1:A3").unwrap(),
            CellRange::new(1, 1, 1, 3)
        );
        assert_eq!(
            CellRange::from("A2:B8").unwrap(),
            CellRange::new(1, 2, 2, 8)
        );
        assert_eq!(
            CellRange::from("D2:D4").unwrap(),
            CellRange::new(4, 2, 4, 4)
        );
        assert_eq!(
            CellRange::from("AA999:AAA1000").unwrap(),
            CellRange::new(27, 999, 703, 1000)
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert!(CellRange::from("").is_none());
        assert!(CellRange::from("?").is_none());
        assert!(CellRange::from("A1").is_none());
        assert!(CellRange::from("A1:").is_none());
        assert!(CellRange::from(":A1").is_none());
        assert!(CellRange::from("1A:A1").is_none());
        assert!(CellRange::from("#ERROR#").is_none());
    }
}
