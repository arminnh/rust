use crate::cell_pos::CellPos;

#[derive(Debug, PartialEq)]
pub enum NumberOrCellPos {
    // TODO: support generic number types -- https://crates.io/crates/num
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

#[cfg(test)]
mod tests {
    use crate::{cell_pos::CellPos, number_or_cell_pos::NumberOrCellPos};

    #[test]
    fn can_parse_cell_range() {
        assert_eq!(
            NumberOrCellPos::from("1").unwrap(),
            NumberOrCellPos::Number(1.0)
        );
        assert_eq!(
            NumberOrCellPos::from("-1").unwrap(),
            NumberOrCellPos::Number(-1.0)
        );
        assert_eq!(
            NumberOrCellPos::from("3.141592").unwrap(),
            NumberOrCellPos::Number(3.141592)
        );
        assert_eq!(
            NumberOrCellPos::from("A1").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new(1, 1))
        );
        assert_eq!(
            NumberOrCellPos::from("ZA99").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new(677, 99))
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert!(NumberOrCellPos::from("").is_none());
        assert!(NumberOrCellPos::from("?").is_none());
        assert!(NumberOrCellPos::from("=123").is_none());
        assert!(NumberOrCellPos::from("Z0").is_none());
        assert!(NumberOrCellPos::from("A1:").is_none());
        assert!(NumberOrCellPos::from(":A1").is_none());
        assert!(NumberOrCellPos::from("1A:A1").is_none());
        assert!(NumberOrCellPos::from("=H8 * Z1").is_none());
        assert!(NumberOrCellPos::from("=9 - 5.795").is_none());
        assert!(NumberOrCellPos::from("#ERROR#").is_none());
    }
}
