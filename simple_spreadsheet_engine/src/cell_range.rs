use crate::cell_pos::CellPos;

#[derive(Debug)]
pub struct CellRange {
    start_cell: CellPos,
    end_cell: CellPos,
}

impl CellRange {
    pub fn from(input: &str) -> Option<Self> {
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

    fn new(start_row: u32, start_col: u32, end_row: u32, end_col: u32) -> Self {
        CellRange {
            start_cell: CellPos::new(start_row, start_col),
            end_cell: CellPos::new(end_row, end_col),
        }
    }
}
