use std::fmt;

use crate::expression::Expression;
use crate::sheet::Sheet;

#[derive(Debug, PartialEq)]
pub enum Cell {
    Empty,
    Error(String),
    Expression(Expression),
    Number(f64),
    Text(String),
}

impl Cell {
    /// Resolve the Cell so it can be displayed. If expression, resolve the expression, otherwise simply
    /// return the cell because the content can be displayed directly.
    pub fn resolve(&self, row: usize, col: usize, sheet: &mut Sheet) {
        match self {
            Cell::Expression(e) => e.resolve(row, col, sheet),
            _ => (),
        }
    }

    pub fn parse(row: usize, col: usize, input: &str) -> Self {
        let trimmed = input.trim();
        if let Some(first_char) = trimmed.chars().next() {
            match first_char {
                '^' | '<' | '>' | '=' => match Expression::parse(row, col, &trimmed[1..]) {
                    Ok(expression) => Cell::Expression(expression),
                    Err(e) => Cell::Error(e.to_string()),
                },
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, ""),
            Cell::Error(e) => write!(f, "#ERROR#: {}", e),
            Cell::Expression(e) => write!(f, "{:?}", e),
            Cell::Number(n) => write!(f, "{}", n),
            Cell::Text(t) => write!(f, "{}", t),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::cell_pos::CellPos;
    use crate::cell_range::CellRange;
    use crate::expression::{Clone, Expression};
    use crate::formula::{ArithmeticOperator, Formula, Operator};
    use crate::function::Function;
    use crate::number_or_cell_pos::NumberOrCellPos;

    #[test]
    fn parses_empty_cell() {
        assert_eq!(Cell::parse(""), Cell::Empty);
    }

    #[test]
    fn parses_text_cells() {
        assert_eq!(Cell::parse("amount"), Cell::Text("amount".to_string()));
        assert_eq!(Cell::parse("Coffee"), Cell::Text("Coffee".to_string()));
        assert_eq!(Cell::parse("Total"), Cell::Text("Total".to_string()));
        assert_eq!(
            Cell::parse("total_price"),
            Cell::Text("total_price".to_string())
        );
        assert_eq!(Cell::parse("#ERROR#"), Cell::Text("#ERROR#".to_string()));
    }

    #[test]
    fn parses_clone_cells() {
        assert_eq!(
            Cell::parse("^"),
            Cell::Expression(Expression::Clone(Clone::Top))
        );

        assert_eq!(
            Cell::parse("<"),
            Cell::Expression(Expression::Clone(Clone::Left))
        );

        assert_eq!(
            Cell::parse(">"),
            Cell::Expression(Expression::Clone(Clone::Right))
        );
    }

    #[test]
    fn parses_formula_cells() {
        assert_eq!(
            Cell::parse("=A1 + B2"),
            Cell::Expression(Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::CellPos(CellPos::new("B2".to_string(), 2, 2))
            )))
        );

        assert_eq!(
            Cell::parse("=A1 - 1"),
            Cell::Expression(Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::Number(1.0)
            )))
        );
    }

    #[test]
    fn parses_function_cells() {
        assert_eq!(
            Cell::parse("=AVG(A1:A3)"),
            Cell::Expression(Expression::Function(Function::Avg(CellRange::new(
                "A1:A3".to_string(),
                1,
                1,
                3,
                1
            ))))
        );

        assert_eq!(
            Cell::parse("=SUM(D2:D4)"),
            Cell::Expression(Expression::Function(Function::Sum(CellRange::new(
                "D2:D4".to_string(),
                2,
                4,
                4,
                4
            ))))
        );
    }

    #[test]
    fn parses_error_cells() {
        println!("{:?}", Cell::parse("=nope + 1"));
        println!("{:?}", Cell::parse("=IF(1, 2, 3)"));
        println!("{:?}", Cell::parse("=LOOKUP(F4, B5:B9, C5:C9)"));
        println!("{:?}", Cell::parse("=DATE(2015, 5, 20)"));
        println!("{:?}", Cell::parse("=AVG(?)"));
    }
}
