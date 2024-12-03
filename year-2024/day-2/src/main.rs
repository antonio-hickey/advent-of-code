/* Day - 2: Red-Nosed Reports */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
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
struct DataModel;
impl DataModel {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        todo!()
    }
}
