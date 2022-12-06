use std::fs;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse_move(c: char) -> Option<Move> {
    match c {
        'A' | 'X' => Some(Move::Rock),
        'B' | 'Y' => Some(Move::Paper),
        'C' | 'Z' => Some(Move::Scissors),
        _ => None,
    }
}

/// The score for a single round is the score for the shape you selected
/// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of
/// the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
fn calculate_score(opponent_move: Move, my_move: Move) -> u32 {
    match opponent_move {
        Move::Rock => match my_move {
            Move::Rock => 3 + 1,
            Move::Paper => 6 + 2,
            Move::Scissors => 0 + 3,
        },
        Move::Paper => match my_move {
            Move::Rock => 0 + 1,
            Move::Paper => 3 + 2,
            Move::Scissors => 6 + 3,
        },
        Move::Scissors => match my_move {
            Move::Rock => 6 + 1,
            Move::Paper => 0 + 2,
            Move::Scissors => 3 + 3,
        },
    }
}

/// What would your total score be if everything goes exactly according to your strategy guide?
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_2/input") {
        let score = contents.lines().fold(0, |acc, line| {
            let opponent_move = parse_move(line.chars().nth(0).unwrap()).unwrap();
            let my_move = parse_move(line.chars().nth(2).unwrap()).unwrap();
            acc + calculate_score(opponent_move, my_move)
        });
        println!("Score is: {}", score);
    }
}
