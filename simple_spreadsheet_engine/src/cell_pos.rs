#[derive(Debug, Eq, PartialEq)]
pub struct CellPos {
    pub str: String,
    pub row: usize,
    pub col: usize,
}

impl CellPos {
    pub fn new(str: String, row: usize, col: usize) -> Self {
        CellPos { str, row, col }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        // TODO: validate and split with regex instead -- https://crates.io/crates/regex
        match input.find(|c: char| c.is_ascii_digit()) {
            Some(i) => {
                if let Ok(row) = input[i..].parse::<usize>() {
                    let mut column: usize = 0;
                    for c in input[..i].chars() {
                        column *= 26;
                        column += match c {
                            'A'..='Z' => c as usize - 'A' as usize + 1,
                            'a'..='z' => c as usize - 'a' as usize + 1,
                            _ => {
                                return Err(format!("Unexpected character '{}'.", c));
                            }
                        }
                    }
                    if column == 0 || row == 0 {
                        Err(format!("Invalid row '{}' or column '{}'.", row, column))
                    } else {
                        Ok(CellPos::new(input.to_string(), row, column))
                    }
                } else {
                    Err(format!(
                        "Could not parse '{}' as column number.",
                        &input[i..]
                    ))
                }
            }
            _ => Err(format!("No digit in '{}'.", input)),
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
        assert_eq!(
            CellPos::parse("A1").unwrap(),
            CellPos::new("A1".to_string(), 1, 1)
        );
        assert_eq!(
            CellPos::parse("a1").unwrap(),
            CellPos::new("a1".to_string(), 1, 1)
        );
        assert_eq!(
            CellPos::parse("E9").unwrap(),
            CellPos::new("E9".to_string(), 9, 5)
        );
        assert_eq!(
            CellPos::parse("C9999999").unwrap(),
            CellPos::new("C9999999".to_string(), 9999999, 3)
        );
        assert_eq!(
            CellPos::parse("Z123").unwrap(),
            CellPos::new("Z123".to_string(), 123, 26)
        );
        assert_eq!(
            CellPos::parse("z99").unwrap(),
            CellPos::new("z99".to_string(), 99, 26)
        );
    }

    #[test]
    fn can_parse_multiple_chars() {
        assert_eq!(
            CellPos::parse("AA1").unwrap(),
            CellPos::new("AA1".to_string(), 1, 27)
        );
        assert_eq!(
            CellPos::parse("AB234").unwrap(),
            CellPos::new("AB234".to_string(), 234, 28)
        );
        assert_eq!(
            CellPos::parse("AZ99").unwrap(),
            CellPos::new("AZ99".to_string(), 99, 52)
        );
        assert_eq!(
            CellPos::parse("ZA100").unwrap(),
            CellPos::new("ZA100".to_string(), 100, 677)
        );
        assert_eq!(
            CellPos::parse("ZZ2").unwrap(),
            CellPos::new("ZZ2".to_string(), 2, 702)
        );
        assert_eq!(
            CellPos::parse("AAA1").unwrap(),
            CellPos::new("AAA1".to_string(), 1, 703)
        );
        assert_eq!(
            CellPos::parse("AAZ1").unwrap(),
            CellPos::new("AAZ1".to_string(), 1, 728)
        );
        assert_eq!(
            CellPos::parse("CCC1").unwrap(),
            CellPos::new("CCC1".to_string(), 1, 2109)
        );
        assert_eq!(
            CellPos::parse("ZZZ1").unwrap(),
            CellPos::new("ZZZ1".to_string(), 1, 18278)
        );
        assert_eq!(
            CellPos::parse("zzz2").unwrap(),
            CellPos::new("zzz2".to_string(), 2, 18278)
        );
    }

    #[test]
    fn handles_unexpected_character() {
        assert_eq!(
            CellPos::parse("#abcd123"),
            Err("Unexpected character '#'.".to_string())
        );
        assert_eq!(
            CellPos::parse("A 1"),
            Err("Unexpected character ' '.".to_string())
        );
        assert_eq!(
            CellPos::parse("=XYZ123"),
            Err("Unexpected character '='.".to_string())
        );
    }

    #[test]
    fn handles_invalid_row_or_column() {
        assert_eq!(
            CellPos::parse("A0"),
            Err("Invalid row '0' or column '1'.".to_string())
        );
        assert_eq!(
            CellPos::parse("1"),
            Err("Invalid row '1' or column '0'.".to_string())
        );
        assert_eq!(
            CellPos::parse("123"),
            Err("Invalid row '123' or column '0'.".to_string())
        );
    }

    #[test]
    fn handles_no_digit_in_input() {
        assert_eq!(CellPos::parse(""), Err("No digit in ''.".to_string()));
        assert_eq!(CellPos::parse("?"), Err("No digit in '?'.".to_string()));
        assert_eq!(CellPos::parse(","), Err("No digit in ','.".to_string()));
        assert_eq!(CellPos::parse("<"), Err("No digit in '<'.".to_string()));
        assert_eq!(
            CellPos::parse("#ERROR#"),
            Err("No digit in '#ERROR#'.".to_string())
        );
        assert_eq!(
            CellPos::parse("average"),
            Err("No digit in 'average'.".to_string())
        );
    }

    #[test]
    fn handles_could_not_parse_column() {
        assert_eq!(
            CellPos::parse("1A"),
            Err("Could not parse '1A' as column number.".to_string())
        );
        assert_eq!(
            CellPos::parse("=A1 - 1"),
            Err("Could not parse '1 - 1' as column number.".to_string())
        );
        assert_eq!(
            CellPos::parse("=AVG(A2:A8)"),
            Err("Could not parse '2:A8)' as column number.".to_string())
        );
    }
}
