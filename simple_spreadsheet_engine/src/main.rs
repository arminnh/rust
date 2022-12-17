use std::fmt;

#[derive(Debug)]
enum NumberOrCellPos {
    Number(f64),
    CellPos(CellPos),
}

impl NumberOrCellPos {
    fn from(input: &str) -> Option<Self> {
        if let Ok(number) = input.parse::<f64>() {
            return Some(NumberOrCellPos::Number(number));
        }

        let mut row: u32 = 1;
        let mut col: u32 = 1;
        for c in input.chars() {
            match c {
                'A'..='Z' => row += c as u32 - 'A' as u32,
                'a'..='z' => row += c as u32 - 'a' as u32,
                _ => match c.to_digit(10) {
                    Some(digit) => col += digit,
                    None => {
                        return None;
                    }
                },
            }
        }

        Some(NumberOrCellPos::CellPos(CellPos::new(row, col)))
    }
}

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
struct Formula {
    operator: Operator,
    left: NumberOrCellPos,
    right: NumberOrCellPos,
}

impl Formula {
    fn from(input: &str) -> Option<Self> {
        match input.split("*").collect::<Vec<&str>>()[..] {
            [lhs, rhs] => match (
                NumberOrCellPos::from(lhs.trim()),
                NumberOrCellPos::from(rhs.trim()),
            ) {
                (Some(left), Some(right)) => Some(Formula {
                    operator: Operator::ArithmeticOperator(ArithmeticOperator::Addition),
                    left,
                    right,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
struct CellPos {
    row: u32,
    col: u32,
}

impl CellPos {
    fn new(row: u32, col: u32) -> Self {
        CellPos { row, col }
    }
}

#[derive(Debug)]
struct CellRange {
    start_cell: CellPos,
    end_cell: CellPos,
}

impl CellRange {
    fn new(start_row: u32, start_col: u32, end_row: u32, end_col: u32) -> Self {
        CellRange {
            start_cell: CellPos::new(start_row, start_col),
            end_cell: CellPos::new(end_row, end_col),
        }
    }
}

#[derive(Debug)]
enum Function {
    AVG(CellRange),
    COUNT(CellRange),
    MAX(CellRange),
    MEDIAN(CellRange),
    MIN(CellRange),
    STDEV(CellRange),
    SUM(CellRange),
}

impl Function {
    fn from(input: &str) -> Option<Self> {
        println!("PARSING Function {}", input);
        None
    }
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
    Formula(Formula),
}

impl Expression {
    fn from(input: &str) -> Option<Self> {
        if let Some(fun) = Function::from(input) {
            return Some(Expression::Function(fun));
        }
        if let Some(formula) = Formula::from(input) {
            return Some(Expression::Formula(formula));
        }
        None
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

#[derive(Debug)]
struct Sheet {
    content: Vec<Vec<Cell>>,
}

impl Sheet {
    fn new() -> Self {
        Sheet { content: vec![] }
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: Vec<String> = self
            .content
            .iter()
            .enumerate()
            .map(|(i, row)| format!("{}: {:?}", i, row))
            .collect();

        write!(f, "{}", out.join("\n"))
    }
}

fn parse_input(input: &str) -> Sheet {
    let mut sheet = Sheet::new();
    input.lines().for_each(|line| {
        let mut row = vec![];
        line.split(',').for_each(|col| {
            row.push(Cell::from(col));
        });
        sheet.content.push(row);
    });
    sheet
}

fn run(input: &str) -> &str {
    println!("{}\n", input);
    let sheet = parse_input(input);
    print!("{}", sheet);

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
