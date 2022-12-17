use crate::cell_range::CellRange;

#[derive(Debug)]
pub enum Function {
    AVG(CellRange),
    COUNT(CellRange),
    MAX(CellRange),
    MEDIAN(CellRange),
    MIN(CellRange),
    STDEV(CellRange),
    SUM(CellRange),
}

impl Function {
    fn parse_name(input: &str) -> Option<fn(CellRange) -> Function> {
        match input {
            "AVG" => Some(Function::AVG),
            "COUNT" => Some(Function::COUNT),
            "MAX" => Some(Function::MAX),
            "MEDIAN" => Some(Function::MEDIAN),
            "MIN" => Some(Function::MIN),
            "STDEV" => Some(Function::STDEV),
            "SUM" => Some(Function::SUM),
            _ => None,
        }
    }

    pub fn from(input: &str) -> Option<Self> {
        match input.split(|c| c == '(' || c == ')').collect::<Vec<&str>>()[..] {
            [function_name, argument, ""] => match (
                Function::parse_name(function_name.trim()),
                CellRange::from(argument.trim()),
            ) {
                (Some(function), Some(cell_range)) => Some(function(cell_range)),
                _ => None,
            },
            _ => None,
        }
    }
}
