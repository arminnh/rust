use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

/// Figure out the signal being sent by the CPU. It has a single register, X, which starts
/// with the value 1. It supports only two instructions:
/// - addx V takes two cycles to complete. After two cycles, the X register is
///   increased by the value V. (V can be negative.)
/// - noop takes one cycle to complete. It has no other effect.
fn part_1(mut lines: Lines<BufReader<File>>) -> i64 {
    let mut cycle = 0;
    let mut x = 1;
    let mut last_addx_value = 0;
    let mut strength = 0;

    loop {
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            strength += cycle * x;
        }

        if last_addx_value != 0 {
            x += last_addx_value;
            last_addx_value = 0;
            continue;
        }

        if let Some(line) = lines.next() {
            match line
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()[..]
            {
                ["addx", num] => {
                    last_addx_value = num.parse::<i64>().unwrap();
                }
                _ => (),
            }
        } else {
            break;
        }
    }

    println!("\n-- END: Cycle '{}', x '{}' -> {}\n", cycle, x, strength);
    strength
}

fn part_2(lines: Lines<BufReader<File>>) {
    println!("Part 2");
}

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Could not open file.")).lines()
}

fn main() {
    part_1(get_lines("inputs/day_10"));
    // part_2(reader.lines());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(get_lines("inputs/day_10_example_1")), 0)
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(get_lines("inputs/day_10_example_2")), 13140)
    }

    #[test]
    fn test_part_2() {}
}
