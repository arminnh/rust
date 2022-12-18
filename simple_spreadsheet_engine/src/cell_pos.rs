#[derive(Debug, PartialEq)]
pub struct CellPos {
    row: u32,
    col: u32,
}

impl CellPos {
    pub fn new(row: u32, col: u32) -> Self {
        CellPos { row, col }
    }

    pub fn from(input: &str) -> Option<Self> {
        // TODO: validate and split with regex instead -- https://crates.io/crates/regex
        match input.find(|c: char| c.is_digit(10)) {
            Some(i) => {
                if let Ok(column) = input[i..].parse::<u32>() {
                    let mut row: u32 = 0;
                    for c in input[..i].chars() {
                        row = row * 26
                            + match c {
                                'A'..='Z' => c as u32 - 'A' as u32 + 1,
                                'a'..='z' => c as u32 - 'a' as u32 + 1,
                                _ => {
                                    return None;
                                }
                            }
                    }
                    if row == 0 || column == 0 {
                        None
                    } else {
                        Some(CellPos::new(row, column))
                    }
                } else {
                    None
                }
            }
            _ => None,
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
        assert_eq!(CellPos::from("A1").unwrap(), CellPos::new(1, 1));
        assert_eq!(CellPos::from("a1").unwrap(), CellPos::new(1, 1));
        assert_eq!(CellPos::from("E9").unwrap(), CellPos::new(5, 9));
        assert_eq!(CellPos::from("C9999999").unwrap(), CellPos::new(3, 9999999));
        assert_eq!(CellPos::from("Z123").unwrap(), CellPos::new(26, 123));
        assert_eq!(CellPos::from("z99").unwrap(), CellPos::new(26, 99));
    }

    #[test]
    fn can_parse_multiple_chars() {
        assert_eq!(CellPos::from("AA1").unwrap(), CellPos::new(27, 1));
        assert_eq!(CellPos::from("AB234").unwrap(), CellPos::new(28, 234));
        assert_eq!(CellPos::from("AZ99").unwrap(), CellPos::new(52, 99));
        assert_eq!(CellPos::from("ZA100").unwrap(), CellPos::new(677, 100));
        assert_eq!(CellPos::from("ZZ2").unwrap(), CellPos::new(702, 2));
        assert_eq!(CellPos::from("AAA1").unwrap(), CellPos::new(703, 1));
        assert_eq!(CellPos::from("AAZ1").unwrap(), CellPos::new(728, 1));
        assert_eq!(CellPos::from("CCC1").unwrap(), CellPos::new(2109, 1));
        assert_eq!(CellPos::from("ZZZ1").unwrap(), CellPos::new(18278, 1));
        assert_eq!(CellPos::from("zzz2").unwrap(), CellPos::new(18278, 2));
    }

    #[test]
    fn handles_invalid_input() {
        assert!(CellPos::from("A0").is_none());
        assert!(CellPos::from("A 1").is_none());
        assert!(CellPos::from("").is_none());
        assert!(CellPos::from("?").is_none());
        assert!(CellPos::from(",").is_none());
        assert!(CellPos::from("1").is_none());
        assert!(CellPos::from("123").is_none());
        assert!(CellPos::from("#abcd123").is_none());
        assert!(CellPos::from("=A1 - 1").is_none());
        assert!(CellPos::from("<").is_none());
        assert!(CellPos::from("=XYZ123").is_none());
        assert!(CellPos::from("#ERROR#").is_none());
        assert!(CellPos::from("=AVG(A2:A8)").is_none());
        assert!(CellPos::from("average").is_none());
    }
}
