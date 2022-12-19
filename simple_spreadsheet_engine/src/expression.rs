use crate::formula::Formula;
use crate::function::Function;
use crate::sheet::Sheet;

#[derive(Debug, PartialEq)]
pub enum Clone {
    Left,
    Right,
    Top,
}

impl Clone {
    fn process(&self, sheet: &Sheet) -> String {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Clone(Clone),
    Function(Function),
    Formula(Formula),
}

impl Expression {
    pub fn process(&self, sheet: &Sheet) -> String {
        match self {
            Expression::Clone(e) => e.process(sheet),
            Expression::Function(e) => e.process(sheet),
            Expression::Formula(e) => e.process(sheet),
        }
    }
}

impl TryFrom<char> for Expression {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Expression::Clone(Clone::Top)),
            '<' => Ok(Expression::Clone(Clone::Left)),
            '>' => Ok(Expression::Clone(Clone::Right)),
            _ => Err("Unsupported expression."),
        }
    }
}

impl TryFrom<&str> for Expression {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input.len() == 1 {
            return Expression::try_from(input.chars().nth(0).unwrap());
        }
        if let Ok(fun) = Function::try_from(input) {
            return Ok(Expression::Function(fun));
        }
        if let Ok(formula) = Formula::try_from(input) {
            return Ok(Expression::Formula(formula));
        }
        Err("Unsupported expression.")
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
            Expression::try_from('^').unwrap(),
            Expression::Clone(Clone::Top)
        );
        assert_eq!(
            Expression::try_from('<').unwrap(),
            Expression::Clone(Clone::Left)
        );
        assert_eq!(
            Expression::try_from('>').unwrap(),
            Expression::Clone(Clone::Right)
        );
        assert_eq!(
            Expression::try_from("^").unwrap(),
            Expression::Clone(Clone::Top)
        );
        assert_eq!(
            Expression::try_from("<").unwrap(),
            Expression::Clone(Clone::Left)
        );
        assert_eq!(
            Expression::try_from(">").unwrap(),
            Expression::Clone(Clone::Right)
        );
    }

    #[test]
    fn can_parse_arithmetic_expressions() {
        assert_eq!(
            Expression::try_from("A1 + B2").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::CellPos(CellPos::new("B2".to_string(), 2, 2))
            ))
        );

        assert_eq!(
            Expression::try_from("9.60 * 0.8").unwrap(),
            Expression::Formula(Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::Number(9.60),
                NumberOrCellPos::Number(0.8)
            ))
        );

        assert_eq!(
            Expression::try_from("A1 - 1").unwrap(),
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
            Expression::try_from("AVG(A1:A3)").unwrap(),
            Expression::Function(Function::AVG(CellRange::new(
                "A1:A3".to_string(),
                1,
                1,
                3,
                1
            )))
        );

        assert_eq!(
            Expression::try_from("COUNT(B2:B11)").unwrap(),
            Expression::Function(Function::COUNT(CellRange::new(
                "B2:B11".to_string(),
                2,
                2,
                11,
                2
            )))
        );

        assert_eq!(
            Expression::try_from("SUM(D2:D4)").unwrap(),
            Expression::Function(Function::SUM(CellRange::new(
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
        assert_eq!(Expression::try_from(""), err);
        assert_eq!(Expression::try_from("v"), err);
        assert_eq!(Expression::try_from('v'), err);
        assert_eq!(Expression::try_from("=1.23 + 456"), err);
        assert_eq!(Expression::try_from("=SUM(D2:D4)"), err);
        assert_eq!(Expression::try_from("IF(1, 2, 3)"), err);
        assert_eq!(Expression::try_from("LOOKUP(F4, B5:B9, C5:C9)"), err);
        assert_eq!(Expression::try_from("DATE(2015, 5, 20)"), err);
        assert_eq!(Expression::try_from("AVG(?)"), err);
        assert_eq!(Expression::try_from("#ERROR#"), err);
    }
}
