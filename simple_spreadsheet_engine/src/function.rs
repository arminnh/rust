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
pub enum FunctionName {
    Avg,
    Count,
    Max,
    Median,
    Min,
    Stdev,
    Sum,
}

impl FunctionName {
    pub fn parse(input: &str) -> Option<FunctionName> {
        match input {
            "AVG" => Some(FunctionName::Avg),
            "COUNT" => Some(FunctionName::Count),
            "MAX" => Some(FunctionName::Max),
            "MEDIAN" => Some(FunctionName::Median),
            "MIN" => Some(FunctionName::Min),
            "STDEV" => Some(FunctionName::Stdev),
            "SUM" => Some(FunctionName::Sum),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    name: FunctionName,
    cell_range: CellRange,
}

impl Function {
    fn new(name: FunctionName, cell_range: CellRange) -> Self {
        Function { name, cell_range }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        match input.split(|c| c == '(' || c == ')').collect::<Vec<&str>>()[..] {
            [function_name, argument, ""] => match (
                FunctionName::parse(function_name.trim()),
                CellRange::parse(argument.trim()),
            ) {
                (Some(function), Ok(cell_range)) => Ok(Function::new(function, cell_range)),
                (Some(_), Err(e)) => {
                    Err(format!("Invalid function argument '{}': '{}'", argument, e))
                }
                _ => Err(format!("Function '{}' not supported.", function_name)),
            },
            _ => Err("Expected '(' and ')' in function.".to_string()),
        }
    }

    pub fn resolve(&self, row: usize, col: usize, sheet: &mut Sheet) {
        let nums: Vec<f64> = self.cell_range.resolve(row, col, sheet);
        let out = self.do_function(nums);
        println!("... {}\n", out);
        sheet.set_resolved(row, col, Cell::Number(out));
    }

    fn do_function(&self, mut nums: Vec<f64>) -> f64 {
        let nums_to_str = |nums: &Vec<f64>| {
            nums.iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        };

        let out: f64 = match self.name {
            FunctionName::Avg => {
                println!("  > =AVG({})", self.cell_range.str);
                println!("... AVG({})", nums_to_str(&nums));
                nums.iter().sum::<f64>() / nums.len() as f64
            }
            FunctionName::Count => {
                println!("  > =COUNT({})", self.cell_range.str);
                println!("... COUNT({})", nums_to_str(&nums));
                nums.len() as f64
            }
            FunctionName::Max => {
                println!("  > =MAX({})", self.cell_range.str);
                println!("... MAX({})", nums_to_str(&nums));
                match nums.iter().max_by(|a, b| a.total_cmp(b)) {
                    Some(max) => *max,
                    None => f64::NAN,
                }
            }
            FunctionName::Median => {
                println!("  > =MEDIAN({})", self.cell_range.str);
                println!("... MEDIAN({})", nums_to_str(&nums));
                nums.sort_by(|a, b| a.total_cmp(b));
                nums[nums.len() / 2]
            }
            FunctionName::Min => {
                println!("  > =MIN({})", self.cell_range.str);
                println!("... MIN({})", nums_to_str(&nums));
                match nums.iter().min_by(|a, b| a.total_cmp(b)) {
                    Some(min) => *min,
                    None => f64::NAN,
                }
            }
            FunctionName::Stdev => {
                println!("  > =STDEV({})", self.cell_range.str);
                println!("... STDEV({})", nums_to_str(&nums));
                match std_deviation(&nums) {
                    Some(stddev) => stddev,
                    None => f64::NAN,
                }
            }
            FunctionName::Sum => {
                println!("  > =SUM({})", self.cell_range.str);
                println!("... SUM({})", nums_to_str(&nums));
                nums.iter().fold(0.0, |acc, n| acc + n)
            }
        };
        out
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
