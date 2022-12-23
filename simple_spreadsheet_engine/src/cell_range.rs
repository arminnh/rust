use crate::{cell::Cell, cell_pos::CellPos, sheet::Sheet};

#[derive(Debug, Eq, PartialEq)]
pub struct CellRange {
    pub str: String,
    pub start_cell: CellPos,
    pub end_cell: CellPos,
}

impl CellRange {
    #[allow(dead_code)]
    pub fn new(
        str: String,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Self {
        let (left_str, right_str) = str.split_once(':').unwrap();
        CellRange {
            str: str.clone(),
            start_cell: CellPos::new(left_str.to_string(), start_row, start_col),
            end_cell: CellPos::new(right_str.to_string(), end_row, end_col),
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        // TODO: don't allow invalid ranges
        // TODO: support absolute reference with $
        match input.split(':').collect::<Vec<&str>>()[..] {
            [lhs, rhs] => match (CellPos::parse(lhs), CellPos::parse(rhs)) {
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

    /// Return a vector of numbers in cells that lie in the specified range.
    pub fn resolve(&self, row: usize, col: usize, sheet: &mut Sheet) -> Vec<f64> {
        let mut out = Vec::new();
        for i in self.start_cell.row..=self.end_cell.row {
            for j in self.start_cell.col..=self.end_cell.col {
                // if !sheet.is_resolved(i - 1, j - 1) {
                //     sheet.cells[i - 1][j - 1].resolve(row, col, sheet);
                // }

                if let Cell::Number(n) = sheet.cells[i - 1][j - 1] {
                    out.push(n)
                }
            }
        }
        out
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
            CellRange::parse("A1:A3").unwrap(),
            CellRange::new("A1:A3".to_string(), 1, 1, 3, 1)
        );
        assert_eq!(
            CellRange::parse("A2:B8").unwrap(),
            CellRange::new("A2:B8".to_string(), 2, 1, 8, 2)
        );
        assert_eq!(
            CellRange::parse("D2:D4").unwrap(),
            CellRange::new("D2:D4".to_string(), 2, 4, 4, 4)
        );
        assert_eq!(
            CellRange::parse("AA999:AAA1000").unwrap(),
            CellRange::new("AA999:AAA1000".to_string(), 999, 27, 1000, 703)
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert_eq!(
            CellRange::parse(""),
            Err("Could not find ':' in cell range ''.".to_string())
        );
        assert_eq!(
            CellRange::parse("?"),
            Err("Could not find ':' in cell range '?'.".to_string())
        );
        assert_eq!(
            CellRange::parse("A1"),
            Err("Could not find ':' in cell range 'A1'.".to_string())
        );
        assert_eq!(
            CellRange::parse("#ERROR#"),
            Err("Could not find ':' in cell range '#ERROR#'.".to_string())
        );
        assert_eq!(
            CellRange::parse("A1:"),
            Err("Right side is not a valid cell range: No digit in ''.".to_string())
        );
        assert_eq!(
            CellRange::parse(":A1"),
            Err("Left side is not a valid cell range: No digit in ''.".to_string())
        );
        assert_eq!(
            CellRange::parse("2X:3Y"),
            Err("Input not a valid cell range: { lhs: 'Could not parse '2X' as column number.', rhs: 'Could not parse '3Y' as column number.' }.".to_string())
        );
        assert_eq!(
            CellRange::parse("1A:A1"),
            Err(
                "Left side is not a valid cell range: Could not parse '1A' as column number."
                    .to_string()
            )
        );
    }
}
