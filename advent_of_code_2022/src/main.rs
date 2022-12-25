use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Item = usize;

fn parse_items(s: String) -> Result<VecDeque<Item>, Box<dyn Error>> {
    Ok(s.split_once(":")
        .ok_or("Could not split on ':'")?
        .1
        .split(",")
        .map(|i| i.trim().parse().expect("Item is not a valid number."))
        .collect())
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(usize),
}

impl Operand {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        if s == "old" {
            Ok(Operand::Old)
        } else {
            Ok(Operand::Num(s.parse()?))
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

    fn execute(&self, x: usize, y: usize) -> usize {
        match self {
            Operator::Add => x + y,
            Operator::Mult => x * y,
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
    fn from_str(s: String) -> Result<Self, Box<dyn Error>> {
        if let [left, op, right] = s
            .split_once("=")
            .ok_or("Could not split on '='")?
            .1
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            Ok(Operation {
                operator: Operator::from_str(op),
                left: Operand::from_str(left)?,
                right: Operand::from_str(right)?,
            })
        } else {
            Err("Invalid operation.".into())
        }
    }

    pub fn execute(&self, old: usize) -> usize {
        match (&self.left, &self.right) {
            (Operand::Old, Operand::Old) => self.operator.execute(old, old),
            (Operand::Old, Operand::Num(num)) => self.operator.execute(old, *num),
            (Operand::Num(num), Operand::Old) => self.operator.execute(*num, old),
            (Operand::Num(left), Operand::Num(right)) => self.operator.execute(*left, *right),
        }
    }
}

#[derive(Debug)]
struct Action {
    denominator: usize,
    target_if_true: i32,
    target_if_false: i32,
}

impl Action {
    fn from_str(s_test: String, s_true: String, s_false: String) -> Result<Self, Box<dyn Error>> {
        Ok(Action {
            denominator: s_test
                .split_once("divisible by")
                .ok_or("Could not split on 'divisible by'")?
                .1
                .trim()
                .parse()?,
            target_if_true: s_true
                .split_once("monkey")
                .ok_or("Could not split on 'monkey'")?
                .1
                .trim()
                .parse()?,
            target_if_false: s_false
                .split_once("monkey")
                .ok_or("Could not split on 'monkey'")?
                .1
                .trim()
                .parse()?,
        })
    }

    pub fn get_target(&self, worry_level: usize) -> &i32 {
        if worry_level % self.denominator == 0 {
            // println!(
            //     "    Current worry level is divisible by {}.",
            //     self.denominator
            // );
            &self.target_if_true
        } else {
            // println!(
            //     "    Current worry level is not divisible by {}.",
            //     self.denominator
            // );
            &self.target_if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    action: Action,
    inspections: i32,
}

impl Monkey {
    fn from_str(lines: &mut Lines<BufReader<File>>) -> Result<Self, Box<dyn Error>> {
        Ok(Monkey {
            items: parse_items(lines.next().ok_or("EOL while reading items")??)?,
            operation: Operation::from_str(lines.next().ok_or("EOL while reading operation")??)?,
            action: Action::from_str(
                lines.next().ok_or("EOL while reading test")??,
                lines.next().ok_or("EOL while reading test true case")??,
                lines.next().ok_or("EOL while reading test false case")??,
            )?,
            inspections: 0,
        })
    }

    fn pop_item(&mut self) -> Option<Item> {
        let mut i = self.items.pop_front()?;
        self.inspections += 1;
        // println!("  Monkey inspects an item with a worry level of {}.", i);
        // print!("    Worry level increases from {}", i);
        i = self.operation.execute(i);
        // println!(" to {}.", i);
        i = i / 3;
        // println!(
        //     "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
        //     i
        // );
        Some(i)
    }

    fn push_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    fn throw_items(&mut self, monkeys: &HashMap<i32, RefCell<Monkey>>) {
        while let Some(item) = self.pop_item() {
            let target: &i32 = self.action.get_target(item);

            // println!(
            //     "    Item with worry level {} is thrown to monkey {}.",
            //     item, target
            // );
            monkeys.get(target).unwrap().borrow_mut().push_item(item);
        }
    }
}

fn part_1(mut lines: Lines<BufReader<File>>) -> i32 {
    let mut monkeys: HashMap<i32, RefCell<Monkey>> = HashMap::new();

    while let Some(Ok(monkey_line)) = lines.next() {
        let i: u32 = monkey_line.chars().nth(7).unwrap().to_digit(10).unwrap();
        if let Ok(monkey) = Monkey::from_str(&mut lines) {
            monkeys.insert(i.try_into().unwrap(), RefCell::new(monkey));
        } else {
            print!("COULD NOT LOAD MONKEY {}.", i);
        }
        lines.next();
    }

    (1..=20).for_each(|round| {
        (0..monkeys.len()).for_each(|i| {
            // println!("Monkey {}:", i);
            if let Some(monkey) = monkeys.get(&i32::try_from(i).unwrap()) {
                monkey.borrow_mut().throw_items(&monkeys);
            }
        });

        println!(
            "\nAfter round {}, the monkeys are holding items with these worry levels:",
            round
        );
        (0..monkeys.len()).for_each(|i| {
            if let Some(monkey) = monkeys.get(&i32::try_from(i).unwrap()) {
                println!("Monkey {:?}", monkey.borrow().items);
            }
        });
    });

    let mut inspections: Vec<i32> = monkeys
        .values()
        .into_iter()
        .map(|v| v.borrow().inspections)
        .collect();
    inspections.sort();
    println!("\n\n{inspections:?}");
    let out = inspections[inspections.len() - 2] * inspections[inspections.len() - 1];

    println!("Result: {out}");
    out
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
    part_1(get_lines("inputs/day_11"));
    // let part_2 = part_2(get_lines("inputs/day_11_example"));
    // println!("{}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(get_lines("inputs/day_11_example")), 10605)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_lines("inputs/day_11")), 100345)
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(get_lines("inputs/day_11")), 0)
    // }
}
