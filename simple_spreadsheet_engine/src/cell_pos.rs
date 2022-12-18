#[derive(Debug, PartialEq)]
pub struct CellPos {
    pub row: usize,
    pub col: usize,
}

impl CellPos {
    pub fn new(row: usize, col: usize) -> Self {
        CellPos { row, col }
    }
}

impl TryFrom<&str> for CellPos {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        // TODO: validate and split with regex instead -- https://crates.io/crates/regex
        match input.find(|c: char| c.is_digit(10)) {
            Some(i) => {
                if let Ok(column) = input[i..].parse::<usize>() {
                    let mut row: usize = 0;
                    for c in input[..i].chars() {
                        row *= 26;
                        row += match c {
                            'A'..='Z' => c as usize - 'A' as usize + 1,
                            'a'..='z' => c as usize - 'a' as usize + 1,
                            _ => {
                                return Err(format!("Unexpected character '{}'.", c));
                            }
                        }
                    }
                    if row == 0 || column == 0 {
                        Err(format!("Invalid row '{}' or column '{}'.", row, column))
                    } else {
                        Ok(CellPos::new(row, column))
                    }
                } else {
                    Err(format!(
                        "Could not parse '{}' as column number.",
                        input[i..].to_string()
                    ))
                }
            }
            _ => Err(format!("No digit in '{}'.", input.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_pos::CellPos;

    // TODO: parametrized tests instead of asserting each variant manually.
    // Could do it through macros or with a package https://crates.io/crates/rstest

    #[test]
    fn can_parse_single_char() {
        assert_eq!(CellPos::try_from("A1").unwrap(), CellPos::new(1, 1));
        assert_eq!(CellPos::try_from("a1").unwrap(), CellPos::new(1, 1));
        assert_eq!(CellPos::try_from("E9").unwrap(), CellPos::new(5, 9));
        assert_eq!(
            CellPos::try_from("C9999999").unwrap(),
            CellPos::new(3, 9999999)
        );
        assert_eq!(CellPos::try_from("Z123").unwrap(), CellPos::new(26, 123));
        assert_eq!(CellPos::try_from("z99").unwrap(), CellPos::new(26, 99));
    }

    #[test]
    fn can_parse_multiple_chars() {
        assert_eq!(CellPos::try_from("AA1").unwrap(), CellPos::new(27, 1));
        assert_eq!(CellPos::try_from("AB234").unwrap(), CellPos::new(28, 234));
        assert_eq!(CellPos::try_from("AZ99").unwrap(), CellPos::new(52, 99));
        assert_eq!(CellPos::try_from("ZA100").unwrap(), CellPos::new(677, 100));
        assert_eq!(CellPos::try_from("ZZ2").unwrap(), CellPos::new(702, 2));
        assert_eq!(CellPos::try_from("AAA1").unwrap(), CellPos::new(703, 1));
        assert_eq!(CellPos::try_from("AAZ1").unwrap(), CellPos::new(728, 1));
        assert_eq!(CellPos::try_from("CCC1").unwrap(), CellPos::new(2109, 1));
        assert_eq!(CellPos::try_from("ZZZ1").unwrap(), CellPos::new(18278, 1));
        assert_eq!(CellPos::try_from("zzz2").unwrap(), CellPos::new(18278, 2));
    }

    #[test]
    fn handles_unexpected_character() {
        assert_eq!(
            CellPos::try_from("#abcd123"),
            Err("Unexpected character '#'.".to_string())
        );
        assert_eq!(
            CellPos::try_from("A 1"),
            Err("Unexpected character ' '.".to_string())
        );
        assert_eq!(
            CellPos::try_from("=XYZ123"),
            Err("Unexpected character '='.".to_string())
        );
    }

    #[test]
    fn handles_invalid_row_or_column() {
        assert_eq!(
            CellPos::try_from("A0"),
            Err("Invalid row '1' or column '0'.".to_string())
        );
        assert_eq!(
            CellPos::try_from("1"),
            Err("Invalid row '0' or column '1'.".to_string())
        );
        assert_eq!(
            CellPos::try_from("123"),
            Err("Invalid row '0' or column '123'.".to_string())
        );
    }

    #[test]
    fn handles_no_digit_in_input() {
        assert_eq!(CellPos::try_from(""), Err("No digit in ''.".to_string()));
        assert_eq!(CellPos::try_from("?"), Err("No digit in '?'.".to_string()));
        assert_eq!(CellPos::try_from(","), Err("No digit in ','.".to_string()));
        assert_eq!(CellPos::try_from("<"), Err("No digit in '<'.".to_string()));
        assert_eq!(
            CellPos::try_from("#ERROR#"),
            Err("No digit in '#ERROR#'.".to_string())
        );
        assert_eq!(
            CellPos::try_from("average"),
            Err("No digit in 'average'.".to_string())
        );
    }

    #[test]
    fn handles_could_not_parse_column() {
        assert_eq!(
            CellPos::try_from("1A"),
            Err("Could not parse '1A' as column number.".to_string())
        );
        assert_eq!(
            CellPos::try_from("=A1 - 1"),
            Err("Could not parse '1 - 1' as column number.".to_string())
        );
        assert_eq!(
            CellPos::try_from("=AVG(A2:A8)"),
            Err("Could not parse '2:A8)' as column number.".to_string())
        );
    }
}
