use crate::expression::{Clone, Expression};

#[derive(Debug)]
pub enum Cell {
    Empty,
    Error,
    Expression(Expression),
    Number(f64),
    Text(String),
}

impl From<&str> for Cell {
    fn from(input: &str) -> Self {
        let trimmed = input.trim();
        if let Some(first_char) = trimmed.chars().nth(0) {
            match first_char {
                '^' => Cell::Expression(Expression::Clone(Clone::Top)),
                '<' => Cell::Expression(Expression::Clone(Clone::Left)),
                '>' => Cell::Expression(Expression::Clone(Clone::Right)),
                '=' => {
                    if let Some(expression) = Expression::from(&trimmed[1..]) {
                        Cell::Expression(expression)
                    } else {
                        Cell::Error
                    }
                }
                _ => {
                    // First try to parse as number
                    if let Ok(num) = trimmed.parse::<f64>() {
                        Cell::Number(num)
                    } else {
                        // Else simply return text
                        Cell::Text(trimmed.to_string())
                    }
                }
            }
        } else {
            Cell::Empty
        }
    }
}
