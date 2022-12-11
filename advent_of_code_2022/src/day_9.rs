use std::collections::HashSet;
use std::fs;
use std::str::Lines;

fn move_head(head: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "U" => (head.0 + 1, head.1),
        "D" => (head.0 - 1, head.1),
        "L" => (head.0, head.1 - 1),
        "R" => (head.0, head.1 + 1),
        _ => panic!("Unsupported direction str."),
    }
}

/// If the head is ever two steps directly up, down, left, or right from the tail,
/// the tail must also move one step in that direction so it remains close enough.
/// Otherwise, if the head and tail aren't touching and aren't in the same row or column,
/// the tail always moves one step diagonally to keep up:
fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if head == tail
        || head.0 == tail.0 && (head.1 - tail.1).abs() < 2
        || head.1 == tail.1 && (head.0 - tail.0).abs() < 2
        || (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1
    {
        *tail
    } else if head.0 == tail.0 {
        (tail.0, tail.1 + if head.1 > tail.1 { 1 } else { -1 })
    } else if head.1 == tail.1 {
        (tail.0 + if head.0 > tail.0 { 1 } else { -1 }, tail.1)
    } else {
        (
            tail.0 + if head.0 > tail.0 { 1 } else { -1 },
            tail.1 + if head.1 > tail.1 { 1 } else { -1 },
        )
    }
}

/// Simulate your complete hypothetical series of motions.
/// How many positions does the tail of the rope visit at least once?
fn part_1(lines: Lines) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    lines.for_each(
        |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, amount] => {
                for _ in 0..amount.parse().unwrap() {
                    head = move_head(head, direction);
                    tail = move_tail(&head, &tail);
                    visited.insert(tail);
                    // println!("{:?}, {:?}", head, tail);
                }
            }
            _ => panic!("Unsupported input: {:?}", line),
        },
    );

    visited.len()
}

fn part_2(lines: Lines) -> i32 {
    println!("Part 2");
    0
}

/// Day 9: Rope Bridge
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_9") {
        println!("{}", part_1(contents.lines()));
        println!("{}", part_2(contents.lines()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT.lines()), 13)
    }

    // #[test]
    // fn test_part_2() {
    //     let input = "...";
    //     assert_eq!(part_2(input.lines()), ())
    // }
}
