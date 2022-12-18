use crate::cell_pos::CellPos;

#[derive(Debug, PartialEq)]
pub enum NumberOrCellPos {
    // TODO: support generic number types -- https://crates.io/crates/num
    Number(f64),
    CellPos(CellPos),
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
            NumberOrCellPos::CellPos(CellPos::new(1, 1))
        );
        assert_eq!(
            NumberOrCellPos::try_from("ZA99").unwrap(),
            NumberOrCellPos::CellPos(CellPos::new(677, 99))
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
