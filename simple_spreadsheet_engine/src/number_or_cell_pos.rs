use crate::cell_pos::CellPos;

#[derive(Debug)]
pub enum NumberOrCellPos {
    Number(f64),
    CellPos(CellPos),
}

impl NumberOrCellPos {
    pub fn from(input: &str) -> Option<Self> {
        if let Ok(number) = input.parse::<f64>() {
            return Some(NumberOrCellPos::Number(number));
        }

        if let Some(pos) = CellPos::from(input) {
            return Some(NumberOrCellPos::CellPos(pos));
        }

        None
    }
}
