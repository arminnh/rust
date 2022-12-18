use crate::number_or_cell_pos::NumberOrCellPos;

// TODO: Add support for % operator. E.g '=A * 10%'
#[derive(Debug, PartialEq)]
pub enum ArithmeticOperator {
    Addition,       // A + B
    Division,       // A / B
    Exponentiation, // A ** B
    Multiplication, // A * B
    Subtraction,    // A - B
}

// TODO: Add support for comparison operations
// #[derive(Debug, PartialEq)]
// pub enum ComparisonOperator {
//     Equal,
//     GreaterThan,
//     GreaterThanOrEqual,
//     LessThan,
//     LessThanOrEqual,
//     NotEqual,
// }

#[derive(Debug, PartialEq)]
pub enum Operator {
    ArithmeticOperator(ArithmeticOperator),
    // TODO: Add support for comparison operations
    // ComparisonOperator(ComparisonOperator),
    // TODO: Add support for concatenation operations
    // TextConcatenationOperator,
}

#[derive(Debug, PartialEq)]
pub struct Formula {
    operator: Operator,
    left: NumberOrCellPos,
    right: NumberOrCellPos,
}

impl Formula {
    pub fn new(operator: Operator, left: NumberOrCellPos, right: NumberOrCellPos) -> Self {
        Formula {
            operator,
            left,
            right,
        }
    }

    pub fn from(input: &str) -> Option<Self> {
        let ops = vec![
            ("**", ArithmeticOperator::Exponentiation),
            ("*", ArithmeticOperator::Multiplication),
            ("/", ArithmeticOperator::Division),
            ("+", ArithmeticOperator::Addition),
            ("-", ArithmeticOperator::Subtraction),
        ];

        for op in ops {
            let formula = match input.split(op.0).collect::<Vec<&str>>()[..] {
                [lhs, rhs] => match (
                    NumberOrCellPos::from(lhs.trim()),
                    NumberOrCellPos::from(rhs.trim()),
                ) {
                    (Some(left), Some(right)) => Some(Formula {
                        operator: Operator::ArithmeticOperator(op.1),
                        left,
                        right,
                    }),
                    _ => None,
                },
                _ => None,
            };

            if let Some(_) = &formula {
                return formula;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_pos::CellPos;
    use crate::formula::{ArithmeticOperator, Formula, Operator};
    use crate::number_or_cell_pos::NumberOrCellPos;

    #[test]
    fn can_parse_arithmetic() {
        assert_eq!(
            Formula::from("1.23 + 456").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::Number(1.23),
                NumberOrCellPos::Number(456.0)
            )
        );

        assert_eq!(
            Formula::from("A1 + B2").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::CellPos(CellPos::new(1, 1)),
                NumberOrCellPos::CellPos(CellPos::new(2, 2))
            )
        );

        assert_eq!(
            Formula::from("C3 / 0 ").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Division),
                NumberOrCellPos::CellPos(CellPos::new(3, 3)),
                NumberOrCellPos::Number(0.0)
            )
        );

        assert_eq!(
            Formula::from("1 / 2").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Division),
                NumberOrCellPos::Number(1.0),
                NumberOrCellPos::Number(2.0)
            )
        );

        assert_eq!(
            Formula::from("0 ** 5").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Exponentiation),
                NumberOrCellPos::Number(0.0),
                NumberOrCellPos::Number(5.0)
            )
        );

        assert_eq!(
            Formula::from("Z20 ** 3").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Exponentiation),
                NumberOrCellPos::CellPos(CellPos::new(26, 20)),
                NumberOrCellPos::Number(3.0)
            )
        );

        assert_eq!(
            Formula::from("9.60 * 0.8").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::Number(9.60),
                NumberOrCellPos::Number(0.8)
            )
        );

        assert_eq!(
            Formula::from("B2 * C2").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::CellPos(CellPos::new(2, 2)),
                NumberOrCellPos::CellPos(CellPos::new(3, 2))
            )
        );

        assert_eq!(
            Formula::from("A1 - 1").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::CellPos(CellPos::new(1, 1)),
                NumberOrCellPos::Number(1.0)
            )
        );

        assert_eq!(
            Formula::from("0 - 3.141592").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::Number(0.0),
                NumberOrCellPos::Number(3.141592)
            )
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert!(Formula::from("=1.23 + 456").is_none());
        assert!(Formula::from("=B2 * C2").is_none());
        assert!(Formula::from("1.23 ++ 456").is_none());
        assert!(Formula::from("+ A1 B2").is_none());
        assert!(Formula::from("C3 0 /").is_none());
        assert!(Formula::from("? 1 2").is_none());
        assert!(Formula::from("0 * 5%").is_none());
        assert!(Formula::from("=SUM(D2:D4)").is_none());
        assert!(Formula::from("=XYZ123").is_none());
        assert!(Formula::from("=nope + 1").is_none());
    }
}
