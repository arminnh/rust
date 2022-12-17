#[derive(Debug)]
enum NumberOrCellRef {
    Number(f64),
    CellRef(Box<Cell>),
}

#[derive(Debug)]
enum MathExpression {
    Addition(NumberOrCellRef, NumberOrCellRef),
    Division(NumberOrCellRef, NumberOrCellRef),
    Multiplication(NumberOrCellRef, NumberOrCellRef),
    Subtraction(NumberOrCellRef, NumberOrCellRef),
}

#[derive(Debug)]
struct CellRange {
    start_row: u64,
    end_row: u64,
    start_column: u64,
    end_column: u64,
}

impl CellRange {
    fn new(start_r: u64, end_r: u64, start_c: u64, end_c: u64) -> Self {
        CellRange {
            start_row: start_r,
            end_row: end_r,
            start_column: start_c,
            end_column: end_c,
        }
    }
}

#[derive(Debug)]
enum Function {
    AVG(CellRange),
    MEDIAN(CellRange),
    STDEV(CellRange),
    SUM(CellRange),
}

#[derive(Debug)]
enum Clone {
    Left,
    Right,
    Top,
}

#[derive(Debug)]
enum Expression {
    Clone(Clone),
    Function(Function),
    MathExpression(MathExpression),
}

impl Expression {
    fn parse_function(input: &str) -> Option<Self> {
        // Some(Expression::Function(Function::AVG(CellRange::new(
        //     1, 2, 3, 4,
        // ))))
        None
    }

    fn parse_math(input: &str) -> Option<Self> {
        Some(Expression::MathExpression(MathExpression::Addition(
            NumberOrCellRef::Number(1.0),
            NumberOrCellRef::Number(2.0),
        )))
    }
}

#[derive(Debug)]
enum Cell {
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
                    // First try to parse function expression
                    if let Some(fun) = Expression::parse_function(input) {
                        Cell::Expression(fun)
                    } else {
                        // Alternatively try to parse math expression
                        if let Some(math) = Expression::parse_math(input) {
                            Cell::Expression(math)
                        } else {
                            // If neither could be parsed, return error
                            Cell::Error
                        }
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

#[derive(Debug)]
struct Sheet {
    content: Vec<Vec<Cell>>,
}

impl Sheet {
    fn new() -> Self {
        Sheet { content: vec![] }
    }
}

fn parse_input(input: &str) -> Sheet {
    let mut sheet = Sheet::new();
    input.lines().for_each(|line| {
        let mut row = vec![];
        line.split(',').for_each(|column| {
            row.push(Cell::from(column));
        });
        sheet.content.push(row);
    });
    sheet
}

fn run(input: &str) -> &str {
    println!("{}\n", input);
    let sheet = parse_input(input);
    println!("{:?}", sheet);

    ""
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "descr  ,amount, unit_price,total_price
Cookies,     4,       2.95,=B2 * C2
Coffee ,     1,=9.60 * 0.8,^
Water  ,     2,       1.20,^
Total  ,      ,           ,=SUM(D2:D4)";

        let output = "descr  ,amount,unit_price,total_price
Cookies,     4,      2.95,       11.8
Coffee ,     1,      7.68,       7.68
Water  ,     2,       1.2,        2.4
Total  ,      ,          ,      21.88";

        assert_eq!(run(input), output);
    }
}
