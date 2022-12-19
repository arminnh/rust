use crate::cell_range::CellRange;

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

    pub fn process(&self, sheet: &crate::sheet::Sheet) -> String {
        let nums_to_str = |nums: &Vec<f64>| {
            nums.iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        };

        let out: String = match self {
            Function::AVG(range) => {
                println!("  > =AVG({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... AVG({})", nums_to_str(&nums));
                (nums.iter().sum::<f64>() / nums.len() as f64).to_string()
            }
            Function::COUNT(range) => {
                println!("  > =COUNT({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... COUNT({})", nums_to_str(&nums));
                nums.len().to_string()
            }
            Function::MAX(range) => {
                println!("  > =MAX({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... MAX({})", nums_to_str(&nums));
                match nums.iter().max_by(|a, b| a.total_cmp(b)) {
                    Some(max) => max.to_string(),
                    None => "None".to_string(),
                }
            }
            Function::MEDIAN(range) => {
                println!("  > =MEDIAN({})", range.str);
                let mut nums: Vec<f64> = range.resolve(sheet);
                println!("... MEDIAN({})", nums_to_str(&nums));
                nums.sort_by(|a, b| a.total_cmp(b));
                nums[nums.len() / 2].to_string()
            }
            Function::MIN(range) => {
                println!("  > =MIN({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... MIN({})", nums_to_str(&nums));
                match nums.iter().min_by(|a, b| a.total_cmp(b)) {
                    Some(min) => min.to_string(),
                    None => "None".to_string(),
                }
            }
            Function::STDEV(range) => {
                println!("  > =STDEV({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... STDEV({})", nums_to_str(&nums));
                match std_deviation(&nums) {
                    Some(stddev) => stddev.to_string(),
                    None => "None".to_string(),
                }
            }
            Function::SUM(range) => {
                println!("  > =SUM({})", range.str);
                let nums: Vec<f64> = range.resolve(sheet);
                println!("... SUM({})", nums_to_str(&nums));
                nums.iter().fold(0.0, |acc, n| acc + n).to_string()
            }
        };

        println!("... {}\n", out);
        out
    }
}

impl TryFrom<&str> for Function {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input.split(|c| c == '(' || c == ')').collect::<Vec<&str>>()[..] {
            [function_name, argument, ""] => match (
                Function::parse_name(function_name.trim()),
                CellRange::try_from(argument.trim()),
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
}

#[cfg(test)]
mod tests {
    use crate::cell_range::CellRange;
    use crate::function::Function;

    #[test]
    fn can_parse_functions() {
        assert_eq!(
            Function::try_from("AVG(A1:A3)").unwrap(),
            Function::AVG(CellRange::new("A1:A3".to_string(), 1, 1, 3, 1))
        );
        assert_eq!(
            Function::try_from("COUNT(B2:B11)").unwrap(),
            Function::COUNT(CellRange::new("B2:B11".to_string(), 2, 2, 11, 2))
        );
        assert_eq!(
            Function::try_from("MAX(A2:A8)").unwrap(),
            Function::MAX(CellRange::new("A2:A8".to_string(), 2, 1, 8, 1))
        );
        assert_eq!(
            Function::try_from("MEDIAN(C1:C3)").unwrap(),
            Function::MEDIAN(CellRange::new("C1:C3".to_string(), 1, 3, 3, 3))
        );
        assert_eq!(
            Function::try_from("MIN(A2:A8)").unwrap(),
            Function::MIN(CellRange::new("A2:A8".to_string(), 2, 1, 8, 1))
        );
        assert_eq!(
            Function::try_from("STDEV(Z1:Z10)").unwrap(),
            Function::STDEV(CellRange::new("Z1:Z10".to_string(), 1, 26, 10, 26))
        );
        assert_eq!(
            Function::try_from("SUM(D2:D4)").unwrap(),
            Function::SUM(CellRange::new("D2:D4".to_string(), 2, 4, 4, 4))
        );
    }

    #[test]
    fn handles_missing_brackets() {
        assert_eq!(
            Function::try_from(""),
            Err("Expected '(' and ')' in function.".to_string())
        );
        assert_eq!(
            Function::try_from("#ERROR#"),
            Err("Expected '(' and ')' in function.".to_string())
        );
    }

    #[test]
    fn handles_unsupported_functions() {
        assert_eq!(
            Function::try_from("=AVG(A1:A3)"),
            Err("Function '=AVG' not supported.".to_string())
        );
        assert_eq!(
            Function::try_from("=SUM(D2:D4)"),
            Err("Function '=SUM' not supported.".to_string())
        );
        assert_eq!(
            Function::try_from("IF(1, 2, 3)"),
            Err("Function 'IF' not supported.".to_string())
        );
        assert_eq!(
            Function::try_from("LOOKUP(F4, B5:B9, C5:C9)"),
            Err("Function 'LOOKUP' not supported.".to_string())
        );
        assert_eq!(
            Function::try_from("DATE(2015, 5, 20)"),
            Err("Function 'DATE' not supported.".to_string())
        );
    }

    #[test]
    fn handles_invalid_arguments() {
        assert_eq!(
            Function::try_from("AVG(?)"),
            Err(
                "Invalid function argument '?': 'Could not find ':' in cell range '?'.'"
                    .to_string()
            )
        );
        assert_eq!(
            Function::try_from("AVG(A1)"),
            Err(
                "Invalid function argument 'A1': 'Could not find ':' in cell range 'A1'.'"
                    .to_string()
            )
        );
        assert_eq!(
            Function::try_from("AVG(A1:)"),
            Err("Invalid function argument 'A1:': 'Right side is not a valid cell range: No digit in ''.'".to_string())
        );
        assert_eq!(
            Function::try_from("AVG(:A1)"),
            Err("Invalid function argument ':A1': 'Left side is not a valid cell range: No digit in ''.'".to_string())
        );
        assert_eq!(
            Function::try_from("AVG(1A:A1)"),
            Err("Invalid function argument '1A:A1': 'Left side is not a valid cell range: Could not parse '1A' as column number.'".to_string())
        );
    }
}
