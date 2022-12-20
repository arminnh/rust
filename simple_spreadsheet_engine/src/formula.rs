use crate::{cell::Cell, number_or_cell_pos::NumberOrCellPos, sheet::Sheet};

// TODO: Add support for % operator. E.g '=A * 10%'
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, Eq, PartialEq)]
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
    pub fn parse(input: &str) -> Result<Self, String> {
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
                    NumberOrCellPos::parse(lhs.trim()),
                    NumberOrCellPos::parse(rhs.trim()),
                ) {
                    (Ok(left), Ok(right)) => Some(Formula {
                        operator: Operator::ArithmeticOperator(op.1),
                        left,
                        right,
                    }),
                    _ => None,
                },
                _ => None,
            };

            if let Some(f) = formula {
                return Ok(f);
            }
        }

        // TODO: propagate the errors of incorrect operands
        Err("Unsupported formula or incorrect operands.".to_string())
    }

    #[allow(dead_code)]
    pub fn new(operator: Operator, left: NumberOrCellPos, right: NumberOrCellPos) -> Self {
        Formula {
            operator,
            left,
            right,
        }
    }

    pub fn resolve(&self, sheet: &Sheet, resolved: &mut Sheet) {
        if let (Some(lhs), Some(rhs)) = (self.left.resolve(sheet), self.right.resolve(sheet)) {
            match &self.operator {
                Operator::ArithmeticOperator(op) => match op {
                    ArithmeticOperator::Addition => {
                        println!("..> ={} + {}", self.left, self.right);
                        println!("... {} + {}", lhs, rhs);
                        let out = lhs + rhs;
                        println!("... {}\n", out);
                        Cell::Number(out);
                    }
                    ArithmeticOperator::Division => {
                        println!("..> ={} / {}", self.left, self.right);
                        println!("... {} / {}", lhs, rhs);
                        let out = lhs / rhs;
                        println!("... {}\n", out);
                        Cell::Number(out);
                    }
                    ArithmeticOperator::Exponentiation => {
                        println!("..> ={} ** {}", self.left, self.right);
                        println!("... {} ** {}", lhs, rhs);
                        let out = f64::powf(lhs, rhs);
                        println!("... {}\n", out);
                        Cell::Number(out);
                    }
                    ArithmeticOperator::Multiplication => {
                        println!("..> ={} * {}", self.left, self.right);
                        println!("... {} * {}", lhs, rhs);
                        let out = lhs * rhs;
                        println!("... {}\n", out);
                        Cell::Number(out);
                    }
                    ArithmeticOperator::Subtraction => {
                        println!("..> ={} - {}", self.left, self.right);
                        println!("... {} - {}", lhs, rhs);
                        let out = lhs - rhs;
                        println!("... {}\n", out);
                        Cell::Number(out);
                    }
                },
            }
        } else {
            Cell::Number(f64::NAN);
        }
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
            Formula::parse("1.23 + 456").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::Number(1.23),
                NumberOrCellPos::Number(456.0)
            )
        );

        assert_eq!(
            Formula::parse("A1 + B2").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::CellPos(CellPos::new("B2".to_string(), 2, 2))
            )
        );

        assert_eq!(
            Formula::parse("C3 / 0 ").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Division),
                NumberOrCellPos::CellPos(CellPos::new("C3".to_string(), 3, 3)),
                NumberOrCellPos::Number(0.0)
            )
        );

        assert_eq!(
            Formula::parse("1 / 2").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Division),
                NumberOrCellPos::Number(1.0),
                NumberOrCellPos::Number(2.0)
            )
        );

        assert_eq!(
            Formula::parse("0 ** 5").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Exponentiation),
                NumberOrCellPos::Number(0.0),
                NumberOrCellPos::Number(5.0)
            )
        );

        assert_eq!(
            Formula::parse("Z20 ** 3").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Exponentiation),
                NumberOrCellPos::CellPos(CellPos::new("Z20".to_string(), 20, 26)),
                NumberOrCellPos::Number(3.0)
            )
        );

        assert_eq!(
            Formula::parse("9.60 * 0.8").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::Number(9.60),
                NumberOrCellPos::Number(0.8)
            )
        );

        assert_eq!(
            Formula::parse("B2 * C2").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Multiplication),
                NumberOrCellPos::CellPos(CellPos::new("B2".to_string(), 2, 2)),
                NumberOrCellPos::CellPos(CellPos::new("C2".to_string(), 2, 3))
            )
        );

        assert_eq!(
            Formula::parse("A1 - 1").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::CellPos(CellPos::new("A1".to_string(), 1, 1)),
                NumberOrCellPos::Number(1.0)
            )
        );

        assert_eq!(
            Formula::parse("0 - 3.141592").unwrap(),
            Formula::new(
                Operator::ArithmeticOperator(ArithmeticOperator::Subtraction),
                NumberOrCellPos::Number(0.0),
                NumberOrCellPos::Number(3.141592)
            )
        );
    }

    #[test]
    fn handles_invalid_input() {
        let err = Err("Unsupported formula or incorrect operands.".to_string());
        assert_eq!(Formula::parse("=1.23 + 456"), err);
        assert_eq!(Formula::parse("=B2 * C2"), err);
        assert_eq!(Formula::parse("1.23 ++ 456"), err);
        assert_eq!(Formula::parse("+ A1 B2"), err);
        assert_eq!(Formula::parse("C3 0 /"), err);
        assert_eq!(Formula::parse("? 1 2"), err);
        assert_eq!(Formula::parse("0 * 5%"), err);
        assert_eq!(Formula::parse("=SUM(D2:D4)"), err);
        assert_eq!(Formula::parse("=XYZ123"), err);
        assert_eq!(Formula::parse("=nope + 1"), err);
    }
}
