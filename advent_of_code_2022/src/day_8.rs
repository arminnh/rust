use std::fs;
use std::str::Lines;

fn print_visbility(visibility: &Vec<Vec<bool>>) {
    println!();
    for row in visibility {
        println!("{:?}", row.iter().map(|&c| c as u8).collect::<Vec<u8>>());
    }
}

/// Consider your map; how many trees are visible from outside the grid?
/// A tree is visible if all of the other trees between it and an edge of
/// the grid are shorter than it. Only consider trees in the same row or column;
/// that is, only look up, down, left, or right from any given tree.
fn part_1(grid: &Vec<Vec<u32>>) -> u32 {
    let size: usize = grid.len();
    let mut visibility: Vec<Vec<bool>> = vec![vec![false; size]; size];

    // Edges start off visible
    (0..size).for_each(|i| {
        visibility[i][0] = true;
        visibility[i][size - 1] = true;
        visibility[0][i] = true;
        visibility[size - 1][i] = true;
    });
    print_visbility(&visibility);

    for i in 1..size - 1 {
        let mut max_left_to_right = grid[i][0];
        let mut max_right_to_left = grid[i][size - 1];
        let mut max_top_to_bottom = grid[0][i];
        let mut max_bottom_to_top = grid[size - 1][i];
        for j in 1..size - 1 {
            if grid[i][j] > max_left_to_right {
                visibility[i][j] = true;
                max_left_to_right = grid[i][j]
            }
            if grid[i][size - 1 - j] > max_right_to_left {
                visibility[i][size - 1 - j] = true;
                max_right_to_left = grid[i][size - 1 - j]
            }
            if grid[j][i] > max_top_to_bottom {
                visibility[j][i] = true;
                max_top_to_bottom = grid[j][i]
            }
            if grid[size - 1 - j][i] > max_bottom_to_top {
                visibility[size - 1 - j][i] = true;
                max_bottom_to_top = grid[size - 1 - j][i]
            }
        }
    }
    print_visbility(&visibility);

    visibility.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, &c| acc + c as u32)
    })
}

fn part_2(grid: &Vec<Vec<u32>>) -> u32 {
    println!("Part 2");
    0
}

fn lines_to_grid(lines: Lines) -> Vec<Vec<u32>> {
    lines
        .map(|l| {
            let row = l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            println!("{:?}", row);
            row
        })
        .collect::<Vec<Vec<u32>>>()
}

/// Day 8: Treetop Tree House
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_8") {
        let grid = lines_to_grid(contents.lines());
        println!("{}", part_1(&grid));
        part_2(&grid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        let grid = lines_to_grid(INPUT.lines());
        assert_eq!(part_1(&grid), 21)
    }

    //     #[test]
    //     fn test_part_2() {
    //         let input = "...";
    //         assert_eq!(part_2(input.lines()), ())
    //     }
}
