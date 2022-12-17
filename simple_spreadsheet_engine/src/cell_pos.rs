#[derive(Debug)]
pub struct CellPos {
    row: u32,
    col: u32,
}

impl CellPos {
    pub fn new(row: u32, col: u32) -> Self {
        CellPos { row, col }
    }

    pub fn from(input: &str) -> Option<Self> {
        // Split on first digit, calculate row from chars before first digit, and column from the digits.
        // TODO: validate and split with regex instead https://crates.io/crates/regex
        match input.find(|c: char| c.is_digit(10)) {
            Some(i) => {
                if let Ok(column) = input[i..].parse::<u32>() {
                    let row: u32 = input[..i].chars().fold(1, |acc, c| {
                        acc + match c {
                            'A'..='Z' => c as u32 - 'A' as u32,
                            'a'..='z' => c as u32 - 'a' as u32,
                            _ => 0,
                        }
                    });

                    Some(CellPos::new(row, column))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
