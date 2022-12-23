use crate::cell::Cell;
use crate::formula::Formula;
use crate::function::Function;
use crate::sheet::Sheet;

#[derive(Debug, PartialEq, Eq)]
pub enum Clone {
    Left,
    Right,
    Top,
}

impl Clone {
    fn resolve(&self, row: usize, col: usize, sheet: &Sheet) {
        let (row, col) = match self {
            Clone::Left => (row, col - 1),
            Clone::Right => (row, col + 1),
            Clone::Top => (row - 1, col),
        };
        let target = &sheet.cells[row][col];
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Clone(usize, usize, Clone),
    Function(Function),
    Formula(Formula),
}

impl Expression {
    pub fn parse(row: usize, col: usize, input: &str) -> Result<Expression, &'static str> {
        if input.len() == 1 {
            match input.chars().next().unwrap() {
                '^' => Ok(Expression::Clone(row, col, Clone::Top)),
                '<' => Ok(Expression::Clone(row, col, Clone::Left)),
                '>' => Ok(Expression::Clone(row, col, Clone::Right)),
                _ => Err("Unsupported expression."),
            }
        } else if let Ok(fun) = Function::parse(input) {
            Ok(Expression::Function(fun))
        } else if let Ok(formula) = Formula::parse(input) {
            Ok(Expression::Formula(formula))
        } else {
            Err("Unsupported expression.")
        }
    }

    pub fn resolve(&self, row: usize, col: usize, sheet: &mut Sheet) {
        match self {
            Expression::Clone(row, col, e) => e.resolve(*row, *col, sheet),
            Expression::Function(e) => e.resolve(row, col, sheet),
            Expression::Formula(e) => e.resolve(row, col, sheet),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_pos::CellPos;
    use crate::cell_range::CellRange;
    use crate::expression::{Clone, Expression};
    use crate::formula::{ArithmeticOperator, Formula, Operator};
    use crate::function::Function;
    use crate::number_or_cell_pos::NumberOrCellPos;

    #[test]
    fn can_parse_clone_expressions() {
        assert_eq!(
            Expression::parse('^').unwrap(),
            Expression::Clone(Clone::Top)
        );
        assert_eq!(
            Expression::parse('<').unwrap(),
            Expression::Clone(Clone::Left)
        );
        assert_eq!(
            Expression::parse('>').unwrap(),
            Expression::Clone(Clone::Right)
        );
        assert_eq!(
            Expression::parse("^").unwrap(),
            Expression::Clone(Clone::Top)
        );
        assert_eq!(
            Expression::parse("<").unwrap(),
            Expression::Clone(Clone::Left)
        );
        assert_eq!(
            Expression::parse(">").unwrap(),
            Expression::Clone(Clone::Right)
        );
    }

    #[test]
    fn can_parse_arithmetic_expressions() {
        assert_eq!(
            Expression::parse("A1 + B2").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::CellPos(CellPos::new("B2".to_string(), 2, 2))
            ))
        );

        assert_eq!(
            Expression::parse("9.60 * 0.8").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::Number(9.60),
                NumberOrCellPos::Number(0.8)
            ))
        );

        assert_eq!(
            Expression::parse("A1 - 1").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::Number(1.0)
            ))
        );
    }

    #[test]
    fn can_parse_function_expressions() {
        assert_eq!(
            Expression::parse("AVG(A1:A3)").unwrap(),
            Expression::Function(Function::Avg(CellRange::new(
                "A1:A3".to_string(),
                1,
                1,
                3,
                1
            )))
        );

        assert_eq!(
            Expression::parse("COUNT(B2:B11)").unwrap(),
            Expression::Function(Function::Count(CellRange::new(
                "B2:B11".to_string(),
                2,
                2,
                11,
                2
            )))
        );

        assert_eq!(
            Expression::parse("SUM(D2:D4)").unwrap(),
            Expression::Function(Function::Sum(CellRange::new(
                "D2:D4".to_string(),
                2,
                4,
                4,
                4
            )))
        );
    }

    #[test]
    fn handles_invalid_input() {
        let err = Err("Unsupported expression.");
        assert_eq!(Expression::parse(""), err);
        assert_eq!(Expression::parse("v"), err);
        assert_eq!(Expression::parse('v'), err);
        assert_eq!(Expression::parse("=1.23 + 456"), err);
        assert_eq!(Expression::parse("=SUM(D2:D4)"), err);
        assert_eq!(Expression::parse("IF(1, 2, 3)"), err);
        assert_eq!(Expression::parse("LOOKUP(F4, B5:B9, C5:C9)"), err);
        assert_eq!(Expression::parse("DATE(2015, 5, 20)"), err);
        assert_eq!(Expression::parse("AVG(?)"), err);
        assert_eq!(Expression::parse("#ERROR#"), err);
    }
}
