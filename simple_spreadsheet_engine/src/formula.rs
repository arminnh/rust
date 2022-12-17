use crate::number_or_cell_pos::NumberOrCellPos;

#[derive(Debug)]
enum ArithmeticOperator {
    Addition,       // A + B
    Division,       // A / B
    Exponentiation, // A ** B
    Multiplication, // A * B
    Subtraction,    // A - B
}

#[derive(Debug)]
enum ComparisonOperator {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

#[derive(Debug)]
enum Operator {
    ArithmeticOperator(ArithmeticOperator),
    ComparisonOperator(ComparisonOperator),
    TextConcatenationOperator,
}

#[derive(Debug)]
pub struct Formula {
    operator: Operator,
    left: NumberOrCellPos,
    right: NumberOrCellPos,
}

impl Formula {
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
