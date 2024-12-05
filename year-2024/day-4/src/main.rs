/* Day - 4: Ceres Search */

use std::{collections::HashMap, fs};

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    let cross_word_puzzle = CrossWordPuzzle::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            cross_word_puzzle
                .horizontal_rows
                .iter()
                .chain(cross_word_puzzle.vertical_rows.iter())
                .chain(cross_word_puzzle.diagonal_rows.iter())
                .fold(i32::default(), |mut acc, row| {
                    row.windows(4).for_each(|window| {
                        let possible_word: String = window.iter().collect();
                        if &possible_word == "XMAS" || &possible_word == "SAMX" {
                            acc += 1;
                        }
                    });

                    acc
                })
        }
        2 => {
            /* Part Two Solution */
            println!("done");
            1
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug, Default)]
/// A model of the puzzle input data
struct CrossWordPuzzle {
    horizontal_rows: Vec<Vec<char>>,
    vertical_rows: Vec<Vec<char>>,
    diagonal_rows: Vec<Vec<char>>,
}
impl CrossWordPuzzle {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        let mut cross_word_puzzle = CrossWordPuzzle::default();
        let mut right_to_left_daigonals: HashMap<i32, Vec<char>> = HashMap::new();
        let mut left_to_right_daigonals: HashMap<usize, Vec<char>> = HashMap::new();

        for (row_idx, row) in puzzle_data.lines().enumerate() {
            cross_word_puzzle.horizontal_rows.push(Vec::new());

            for (col_idx, char) in row.chars().enumerate() {
                if cross_word_puzzle.vertical_rows.len() <= col_idx {
                    cross_word_puzzle.vertical_rows.push(Vec::new());
                }

                cross_word_puzzle.horizontal_rows[row_idx].push(char);
                cross_word_puzzle.vertical_rows[col_idx].push(char);

                right_to_left_daigonals
                    .entry(row_idx as i32 - col_idx as i32)
                    .or_default()
                    .push(char);

                left_to_right_daigonals
                    .entry(row_idx + col_idx)
                    .or_default()
                    .push(char);
            }
        }

        for val in right_to_left_daigonals
            .values()
            .chain(left_to_right_daigonals.values())
        {
            cross_word_puzzle.diagonal_rows.push(val.to_vec());
        }

        cross_word_puzzle
    }
}
