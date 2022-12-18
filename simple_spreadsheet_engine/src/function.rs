use crate::cell_range::CellRange;

// TODO: all multiple argument support
// TODO: add all the functions!
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::cell_range::CellRange;
    use crate::function::Function;

    #[test]
    fn can_parse_functions() {
        assert_eq!(
            Function::from("AVG(A1:A3)").unwrap(),
            Function::AVG(CellRange::new(1, 1, 1, 3))
        );
        assert_eq!(
            Function::from("COUNT(B2:B11)").unwrap(),
            Function::COUNT(CellRange::new(2, 2, 2, 11))
        );
        assert_eq!(
            Function::from("MAX(A2:A8)").unwrap(),
            Function::MAX(CellRange::new(1, 2, 1, 8))
        );
        assert_eq!(
            Function::from("MEDIAN(C1:C3)").unwrap(),
            Function::MEDIAN(CellRange::new(3, 1, 3, 3))
        );
        assert_eq!(
            Function::from("MIN(A2:A8)").unwrap(),
            Function::MIN(CellRange::new(1, 2, 1, 8))
        );
        assert_eq!(
            Function::from("STDEV(Z1:Z10)").unwrap(),
            Function::STDEV(CellRange::new(26, 1, 26, 10))
        );
        assert_eq!(
            Function::from("SUM(D2:D4)").unwrap(),
            Function::SUM(CellRange::new(4, 2, 4, 4))
        );
    }

    #[test]
    fn handles_invalid_input() {
        assert!(Function::from("").is_none());
        assert!(Function::from("=AVG(A1:A3)").is_none());
        assert!(Function::from("=SUM(D2:D4)").is_none());
        assert!(Function::from("IF(1, 2, 3)").is_none());
        assert!(Function::from("LOOKUP(F4, B5:B9, C5:C9)").is_none());
        assert!(Function::from("DATE(2015, 5, 20)").is_none());
        assert!(Function::from("AVG(?)").is_none());
        assert!(Function::from("AVG(A1)").is_none());
        assert!(Function::from("AVG(A1:)").is_none());
        assert!(Function::from("AVG(:A1)").is_none());
        assert!(Function::from("AVG(1A:A1)").is_none());
        assert!(Function::from("#ERROR#").is_none());
    }
}
