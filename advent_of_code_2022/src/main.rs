use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Item = i32;

fn parse_items(s: String) -> Vec<Item> {
    s.split_once(":")
        .expect("Expected ':' while parsing starting items.")
        .1
        .split(",")
        .map(|i| i.trim().parse().expect("Item is not a valid number."))
        .collect()
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(i32),
}

impl Operand {
    fn from_str(s: &str) -> Self {
        if s == "old" {
            Operand::Old
        } else {
            Operand::Num(s.parse().expect("Num operand not a valid number."))
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

impl Operator {
    fn from_str(s: &str) -> Self {
        if s == "*" {
            Operator::Mult
        } else {
            Operator::Add
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

impl Operation {
    fn from_str(s: String) -> Self {
        match s
            .split_once("=")
            .expect("Expected '=' while parsing operation.")
            .1
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            [left, op, right] => Operation {
                operator: Operator::from_str(op),
                left: Operand::from_str(left),
                right: Operand::from_str(right),
            },
            _ => panic!("Invalid operation."),
        }
    }
}

#[derive(Debug)]
struct Action {
    divisible_by: i32,
    target_if_true: i32,
    target_if_false: i32,
}

impl Action {
    fn from_str(s_test: String, s_true: String, s_false: String) -> Self {
        let divisible_by = s_test
            .split_once("divisible by")
            .expect("Expected 'divisible by' while parsing test.")
            .1
            .trim()
            .parse()
            .expect("Denominator not a valid number.");
        let target_if_true = s_true
            .split_once("monkey")
            .expect("Expected 'monkey' while parsing test true case.")
            .1
            .trim()
            .parse()
            .expect("Target monkey not a valid number.");
        let target_if_false = s_false
            .split_once("monkey")
            .expect("Expected 'monkey' while parsing test false case.")
            .1
            .trim()
            .parse()
            .expect("Target monkey not a valid number.");

        Action {
            divisible_by,
            target_if_true,
            target_if_false,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    action: Action,
}

impl Monkey {
    fn from_str(lines: &mut Lines<BufReader<File>>) -> Result<Self, Box<dyn Error>> {
        Ok(Monkey {
            items: parse_items(lines.next().unwrap().unwrap()),
            operation: Operation::from_str(lines.next().unwrap().unwrap()),
            action: Action::from_str(
                lines.next().unwrap().unwrap(),
                lines.next().unwrap().unwrap(),
                lines.next().unwrap().unwrap(),
            ),
        })
    }
}

fn part_1(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut monkeys: HashMap<i32, Monkey> = HashMap::new();

    while let Some(Ok(monkey_line)) = lines.next() {
        let i: u32 = monkey_line.chars().nth(7).unwrap().to_digit(10).unwrap();
        if let Ok(monkey) = Monkey::from_str(&mut lines) {
            monkeys.insert(i.try_into().unwrap(), monkey);
        } else {
            print!("COULD NOT LOAD MONKEY {}.", i);
        }
        lines.next();
    }

    println!("{:#?}", monkeys);
    0
}

// fn part_2(mut lines: Lines<BufReader<File>>) {
// }

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Could not open file.")).lines()
}

// fn next_line(lines: &mut Lines<BufReader<File>>, desc: &str) -> String {
//     lines
//         .next()
//         .expect(&format!("{}{}", "End of file while reading ", desc)[..])
//         .expect(&format!("{}{}", "Error while reading starting items", desc)[..])
// }

fn main() {
    part_1(get_lines("inputs/day_11_example"));
    // let part_2 = part_2(get_lines("inputs/day_11_example"));
    // println!("{}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(get_lines("inputs/day_11_example")), 0)
    }

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(part_1(get_lines("inputs/day_11")), 0)
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(get_lines("inputs/day_11")), 0)
    // }
}
