use std::fs;
use std::str::Lines;

fn part_1(lines: Lines) {
    println!("Part 1");
}

fn part_2(lines: Lines) {
    println!("Part 2");
}

fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_X") {
        part_1(contents.lines());
        part_2(contents.lines());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "...";
        assert_eq!(part_1(input.lines()), ())
    }

    #[test]
    fn test_part_2() {
        let input = "...";
        assert_eq!(part_2(input.lines()), ())
    }
}
