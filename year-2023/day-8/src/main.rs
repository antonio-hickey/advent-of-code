/* Day - 8: Haunted Wasteland */

use std::{collections::BTreeMap, fs, iter::successors};

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    //println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    // parse the puzzle input into a useful model
    let escape_map = EscapeMap::from_puzzle_input(input);

    // create a non ending sequence of instructions
    let mut instructions = escape_map.instructions.iter().cycle();

    match part {
        1 => {
            /* Part One Solution */
            // Loop over the escape map guide and instructions until
            // the next location is "ZZZ" (our escape point). Where each
            // iteration in the loop is computed based on the previous iteration.
            // Finally count how many iterations we looped through (our steps).
            successors(Some("AAA"), |&current_location| {
                // Get the possible locations to go to next using our current location
                // which starts at "AAA", but changes with each iteration
                escape_map.guide.get(current_location).map(|(left, right)| {
                    // Check the next instuction to see if we go to the left
                    // or right location for the next iteration
                    match instructions.next() {
                        Some(Direction::Left) => left.as_str(),
                        Some(Direction::Right) => right.as_str(),
                        _ => panic!("only 2 possible directions brooo"),
                    }
                })
            })
            .take_while(|next_location| next_location != &"ZZZ") // Keep going until reach exit
            .count() as i64 // Count how many steps it took
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A useful model of our puzzle data input (Escape Map)
struct EscapeMap {
    instructions: Vec<Direction>,
    guide: BTreeMap<String, (String, String)>,
}
impl EscapeMap {
    /// Parse our puzzle input into an `EscapeMap`
    fn from_puzzle_input(input: &str) -> Self {
        let mut lines = input.lines();

        Self {
            // Parse out the direction instructions from the puzzle data
            instructions: lines
                .next()
                .unwrap()
                .chars()
                .map(|ch| match ch {
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    _ => panic!("only 2 possible directions brooo"),
                })
                .collect(),
            // Parse out the location guide from the puzzle data
            guide: lines
                .map(|line| {
                    let (location, possible_next_locations) = line.split_once("=").unwrap();
                    let possible_next_locations: Vec<&str> = possible_next_locations
                        .split(", ")
                        .map(|location| location.trim_matches(|c| c == ' ' || c == ')'))
                        .collect();
                    (
                        location.trim_end().to_string(),
                        (
                            possible_next_locations[0].replace('(', "").to_string(),
                            possible_next_locations[1].to_string(),
                        ),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
/// A useful model of the direction instruction in the puzzle
enum Direction {
    Left,
    Right,
}
