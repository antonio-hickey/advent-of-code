/* Day - 1: Historian Hysteria */

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
    let mut location_lists = LocationLists::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            let mut deltas = Vec::new();

            location_lists.left.sort();
            location_lists.right.sort();

            let list_length = location_lists.left.len();
            for idx in 0..list_length {
                let left_location = location_lists.left[idx];
                let right_location = location_lists.right[idx];

                let delta = (left_location - right_location).abs();
                deltas.push(delta);
            }

            deltas.iter().sum()
        }
        2 => {
            /* Part Two Solution */
            let mut similarity_scores: Vec<i64> = Vec::new();

            for left_location in location_lists.left.iter() {
                let mut right_list_occurance_count = 0;

                // TODO: def a better solution than looping
                // this for each location in left list O(n^2)
                for right_location in location_lists.right.iter() {
                    if right_location == left_location {
                        right_list_occurance_count += 1;
                    }
                }

                similarity_scores.push(left_location * right_list_occurance_count);
            }

            similarity_scores.iter().sum()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug, Default)]
/// A model of the puzzle input data
struct LocationLists {
    left: Vec<i64>,
    right: Vec<i64>,
}
impl LocationLists {
    /// Parse the puzzle input data into a meaningful model.
    ///
    /// For this problem just need to split the data into 2
    /// seperate lists, a left and right list.
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        let mut location_lists = LocationLists::default();
        puzzle_data.lines().for_each(|line| {
            let (left_location_id, right_location_id) = line
                .split_once("   ")
                .expect("AoC not to have bad formatted data");

            location_lists
                .left
                .push(left_location_id.parse().expect("AoC not to have bad data"));

            location_lists
                .right
                .push(right_location_id.parse().expect("AoC not to have bad data"));
        });

        location_lists
    }
}
