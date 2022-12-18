use crate::formula::Formula;
use crate::function::Function;

#[derive(Debug, PartialEq)]
pub enum Clone {
    Left,
    Right,
    Top,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Clone(Clone),
    Function(Function),
    Formula(Formula),
}

// impl TryFrom<char> for Expression {
//     fn try_from(c: char) -> Result<Self, Self::Error> {
//         match c {
//             '^' => Ok(Expression::Clone(Clone::Top)),
//             '<' => Ok(Expression::Clone(Clone::Left)),
//             '>' => Ok(Expression::Clone(Clone::Right)),
//             _ => Err("???")
//         }
//     }
// }

impl Expression {
    pub fn from(input: &str) -> Option<Self> {
        if let Some(fun) = Function::from(input) {
            return Some(Expression::Function(fun));
        }
        if let Some(formula) = Formula::from(input) {
            return Some(Expression::Formula(formula));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_pos::CellPos;
    use crate::cell_range::CellRange;
    use crate::expression::Expression;
    use crate::formula::{ArithmeticOperator, Formula, Operator};
    use crate::function::Function;
    use crate::number_or_cell_pos::NumberOrCellPos;

    #[test]
    fn can_parse_expressions() {
        assert_eq!(
            Expression::from("A1 + B2").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::CellPos(CellPos::new(1, 1)),
                NumberOrCellPos::CellPos(CellPos::new(2, 2))
            ))
        );

        assert_eq!(
            Expression::from("9.60 * 0.8").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::Number(9.60),
                NumberOrCellPos::Number(0.8)
            ))
        );

        assert_eq!(
            Expression::from("A1 - 1").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::CellPos(CellPos::new(1, 1)),
                NumberOrCellPos::Number(1.0)
            ))
        );

        assert_eq!(
            Expression::from("AVG(A1:A3)").unwrap(),
            Expression::Function(Function::AVG(CellRange::new(1, 1, 1, 3)))
        );

        assert_eq!(
            Expression::from("COUNT(B2:B11)").unwrap(),
            Expression::Function(Function::COUNT(CellRange::new(2, 2, 2, 11)))
        );

        assert_eq!(
            Expression::from("SUM(D2:D4)").unwrap(),
            Expression::Function(Function::SUM(CellRange::new(4, 2, 4, 4)))
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert!(Expression::from("").is_none());
        assert!(Expression::from("^").is_none());
        assert!(Expression::from("<").is_none());
        assert!(Expression::from(">").is_none());
        assert!(Expression::from("=1.23 + 456").is_none());
        assert!(Expression::from("=SUM(D2:D4)").is_none());
        assert!(Expression::from("IF(1, 2, 3)").is_none());
        assert!(Expression::from("LOOKUP(F4, B5:B9, C5:C9)").is_none());
        assert!(Expression::from("DATE(2015, 5, 20)").is_none());
        assert!(Expression::from("AVG(?)").is_none());
        assert!(Expression::from("#ERROR#").is_none());
    }
}
