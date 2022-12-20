use crate::{cell::Cell, cell_range::CellRange, sheet::Sheet};

fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    match data.len() {
        count if count > 0 => {
            let avg = data.iter().sum::<f64>() / count as f64;
            let variance = data
                .iter()
                .map(|value| {
                    let diff = avg - (*value as f64);
                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

// TODO: all multiple argument support -- probably better through struct of enum FunctionName and vector of args
// TODO: add all the functions!
#[derive(Debug, PartialEq, Eq)]
pub enum Function {
    Avg(CellRange),
    Count(CellRange),
    Max(CellRange),
    Median(CellRange),
    Min(CellRange),
    Stdev(CellRange),
    Sum(CellRange),
}

impl Function {
    fn parse_name(input: &str) -> Option<fn(CellRange) -> Function> {
        match input {
            "AVG" => Some(Function::Avg),
            "COUNT" => Some(Function::Count),
            "MAX" => Some(Function::Max),
            "MEDIAN" => Some(Function::Median),
            "MIN" => Some(Function::Min),
            "STDEV" => Some(Function::Stdev),
            "SUM" => Some(Function::Sum),
            _ => None,
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        match input.split(|c| c == '(' || c == ')').collect::<Vec<&str>>()[..] {
            [function_name, argument, ""] => match (
                Function::parse_name(function_name.trim()),
                CellRange::parse(argument.trim()),
            ) {
                (Some(function), Ok(cell_range)) => Ok(function(cell_range)),
                (Some(_), Err(e)) => {
                    Err(format!("Invalid function argument '{}': '{}'", argument, e))
                }
                _ => Err(format!("Function '{}' not supported.", function_name)),
            },
            _ => Err("Expected '(' and ')' in function.".to_string()),
        }
    }

    pub fn resolve(&self, sheet: &Sheet, resolved: &mut Sheet) {
        let nums_to_str = |nums: &Vec<f64>| {
            nums.iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        };

        let out: f64 = match self {
            Function::Avg(range) => {
                println!("  > =AVG({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... AVG({})", nums_to_str(&nums));
                nums.iter().sum::<f64>() / nums.len() as f64
            }
            Function::Count(range) => {
                println!("  > =COUNT({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... COUNT({})", nums_to_str(&nums));
                nums.len() as f64
            }
            Function::Max(range) => {
                println!("  > =MAX({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... MAX({})", nums_to_str(&nums));
                match nums.iter().max_by(|a, b| a.total_cmp(b)) {
                    Some(max) => *max,
                    None => f64::NAN,
                }
            }
            Function::Median(range) => {
                println!("  > =MEDIAN({})", range.str);
                let mut nums: Vec<f64> = range.resolve(sheet);
                println!("... MEDIAN({})", nums_to_str(&nums));
                nums.sort_by(|a, b| a.total_cmp(b));
                nums[nums.len() / 2]
            }
            Function::Min(range) => {
                println!("  > =MIN({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... MIN({})", nums_to_str(&nums));
                match nums.iter().min_by(|a, b| a.total_cmp(b)) {
                    Some(min) => *min,
                    None => f64::NAN,
                }
            }
            Function::Stdev(range) => {
                println!("  > =STDEV({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... STDEV({})", nums_to_str(&nums));
                match std_deviation(&nums) {
                    Some(stddev) => stddev,
                    None => f64::NAN,
                }
            }
            Function::Sum(range) => {
                println!("  > =SUM({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... SUM({})", nums_to_str(&nums));
                nums.iter().fold(0.0, |acc, n| acc + n)
            }
        };

        println!("... {}\n", out);
        Cell::Number(out);
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_range::CellRange;
    use crate::function::Function;

    #[test]
    fn can_parse_functions() {
        assert_eq!(
            Function::parse("AVG(A1:A3)").unwrap(),
            Function::Avg(CellRange::new("A1:A3".to_string(), 1, 1, 3, 1))
        );
        assert_eq!(
            Function::parse("COUNT(B2:B11)").unwrap(),
            Function::Count(CellRange::new("B2:B11".to_string(), 2, 2, 11, 2))
        );
        assert_eq!(
            Function::parse("MAX(A2:A8)").unwrap(),
            Function::Max(CellRange::new("A2:A8".to_string(), 2, 1, 8, 1))
        );
        assert_eq!(
            Function::parse("MEDIAN(C1:C3)").unwrap(),
            Function::Median(CellRange::new("C1:C3".to_string(), 1, 3, 3, 3))
        );
        assert_eq!(
            Function::parse("MIN(A2:A8)").unwrap(),
            Function::Min(CellRange::new("A2:A8".to_string(), 2, 1, 8, 1))
        );
        assert_eq!(
            Function::parse("STDEV(Z1:Z10)").unwrap(),
            Function::Stdev(CellRange::new("Z1:Z10".to_string(), 1, 26, 10, 26))
        );
        assert_eq!(
            Function::parse("SUM(D2:D4)").unwrap(),
            Function::Sum(CellRange::new("D2:D4".to_string(), 2, 4, 4, 4))
        );
    }

    #[test]
    fn handles_missing_brackets() {
        assert_eq!(
            Function::parse(""),
            Err("Expected '(' and ')' in function.".to_string())
        );
        assert_eq!(
            Function::parse("#ERROR#"),
            Err("Expected '(' and ')' in function.".to_string())
        );
    }

    #[test]
    fn handles_unsupported_functions() {
        assert_eq!(
            Function::parse("=AVG(A1:A3)"),
            Err("Function '=AVG' not supported.".to_string())
        );
        assert_eq!(
            Function::parse("=SUM(D2:D4)"),
            Err("Function '=SUM' not supported.".to_string())
        );
        assert_eq!(
            Function::parse("IF(1, 2, 3)"),
            Err("Function 'IF' not supported.".to_string())
        );
        assert_eq!(
            Function::parse("LOOKUP(F4, B5:B9, C5:C9)"),
            Err("Function 'LOOKUP' not supported.".to_string())
        );
        assert_eq!(
            Function::parse("DATE(2015, 5, 20)"),
            Err("Function 'DATE' not supported.".to_string())
        );
    }

    #[test]
    fn handles_invalid_arguments() {
        assert_eq!(
            Function::parse("AVG(?)"),
            Err(
                "Invalid function argument '?': 'Could not find ':' in cell range '?'.'"
                    .to_string()
            )
        );
        assert_eq!(
            Function::parse("AVG(A1)"),
            Err(
                "Invalid function argument 'A1': 'Could not find ':' in cell range 'A1'.'"
                    .to_string()
            )
        );
        assert_eq!(
            Function::parse("AVG(A1:)"),
            Err("Invalid function argument 'A1:': 'Right side is not a valid cell range: No digit in ''.'".to_string())
        );
        assert_eq!(
            Function::parse("AVG(:A1)"),
            Err("Invalid function argument ':A1': 'Left side is not a valid cell range: No digit in ''.'".to_string())
        );
        assert_eq!(
            Function::parse("AVG(1A:A1)"),
            Err("Invalid function argument '1A:A1': 'Left side is not a valid cell range: Could not parse '1A' as column number.'".to_string())
        );
    }
}
