/* Day - 9: Mirage Maintenance */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    // parse the puzzle input into a useful model
    let oasis_report = OasisReport::from_puzzle_input(input);
    println!("{oasis_report:?}");

    match part {
        1 => {
            /* Part One Solution */
            todo!()
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A model of the puzzle input data
struct OasisReport {
    sensor_history: Vec<Vec<i64>>
}
impl OasisReport {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        Self {
            // Go over each line in the puzzle data
            // and split up each character in the line
            // mapping them into an integer
            sensor_history: puzzle_data
                .lines()
                .map(|line| {
                    line
                        .split(' ')
                        .map(|number| number.parse().unwrap())
                        .collect()
                })
                .collect()
        }
    }
}
