use crate::cell_pos::CellPos;

#[derive(Debug, PartialEq)]
pub struct CellRange {
    pub str: String,
    pub start_cell: CellPos,
    pub end_cell: CellPos,
}

impl CellRange {
    pub fn new(
        str: String,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Self {
        CellRange {
            str,
            start_cell: CellPos::new(start_row, start_col),
            end_cell: CellPos::new(end_row, end_col),
        }
    }

    /// Return a vector of numbers in cells that lie in the specified range.
    /// Only works for cells that contain a number.
    /// TODO: resolve values through a dependency graph to ensure all references can resolve successfully?
    pub(crate) fn resolve(&self, sheet: &crate::sheet::Sheet) -> Vec<f64> {
        let mut out = Vec::new();
        for i in self.start_cell.row..=self.end_cell.row {
            for j in self.start_cell.col..=self.end_cell.col {
                match sheet.content[i - 1][j - 1] {
                    crate::cell::Cell::Number(n) => out.push(n),
                    _ => (),
                }
            }
        }
        out
    }
}

impl TryFrom<&str> for CellRange {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        // TODO: don't allow invalid ranges
        // TODO: support absolute reference with $
        match input.split(':').collect::<Vec<&str>>()[..] {
            [lhs, rhs] => match (CellPos::try_from(lhs), CellPos::try_from(rhs)) {
                (Ok(start_cell), Ok(end_cell)) => Ok(CellRange {
                    str: lhs.to_owned() + ":" + rhs,
                    start_cell,
                    end_cell,
                }),
                (Ok(_), Err(e)) => Err(format!("Right side is not a valid cell range: {}", e)),
                (Err(e), Ok(_)) => Err(format!("Left side is not a valid cell range: {}", e)),
                (Err(e1), Err(e2)) => Err(format!(
                    "Input not a valid cell range: {{ lhs: '{}', rhs: '{}' }}.",
                    e1, e2
                )),
            },
            _ => Err(format!("Could not find ':' in cell range '{}'.", input)),
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
            CellRange::try_from("A1:A3").unwrap(),
            CellRange::new("A1:A3".to_string(), 1, 1, 1, 3)
        );
        assert_eq!(
            CellRange::try_from("A2:B8").unwrap(),
            CellRange::new("A2:B8".to_string(), 1, 2, 2, 8)
        );
        assert_eq!(
            CellRange::try_from("D2:D4").unwrap(),
            CellRange::new("D2:D4".to_string(), 4, 2, 4, 4)
        );
        assert_eq!(
            CellRange::try_from("AA999:AAA1000").unwrap(),
            CellRange::new("AA999:AAA1000".to_string(), 27, 999, 703, 1000)
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert_eq!(
            CellRange::try_from(""),
            Err("Could not find ':' in cell range ''.".to_string())
        );
        assert_eq!(
            CellRange::try_from("?"),
            Err("Could not find ':' in cell range '?'.".to_string())
        );
        assert_eq!(
            CellRange::try_from("A1"),
            Err("Could not find ':' in cell range 'A1'.".to_string())
        );
        assert_eq!(
            CellRange::try_from("#ERROR#"),
            Err("Could not find ':' in cell range '#ERROR#'.".to_string())
        );
        assert_eq!(
            CellRange::try_from("A1:"),
            Err("Right side is not a valid cell range: No digit in ''.".to_string())
        );
        assert_eq!(
            CellRange::try_from(":A1"),
            Err("Left side is not a valid cell range: No digit in ''.".to_string())
        );
        assert_eq!(
            CellRange::try_from("2X:3Y"),
            Err("Input not a valid cell range: { lhs: 'Could not parse '2X' as column number.', rhs: 'Could not parse '3Y' as column number.' }.".to_string())
        );
        assert_eq!(
            CellRange::try_from("1A:A1"),
            Err(
                "Left side is not a valid cell range: Could not parse '1A' as column number."
                    .to_string()
            )
        );
    }
}
