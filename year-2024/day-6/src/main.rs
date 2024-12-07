/* Day - 6: Guard Gallivant */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    // parse the puzzle input into a useful model
    let mut guard_map = GuardMap::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            let mut guards_current_position = guard_map.starting_position;
            let mut guards_current_direction = '^';
            let directions = ['^', '>', 'v', '<'];

            loop {
                let mut direction_idx = directions
                    .iter()
                    .position(|&d| d == guards_current_direction)
                    .expect("Guard to always have a valid direction");

                let next_coordinate = match guards_current_direction {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => panic!("Invalid direction"),
                };

                let next_location = (
                    guards_current_position.0 + next_coordinate.0,
                    guards_current_position.1 + next_coordinate.1,
                );

                if guard_map.obstructions.contains(&next_location) {
                    direction_idx = (direction_idx + 1) % directions.len();
                    guards_current_direction = directions[direction_idx];
                    continue;
                } else {
                    guard_map.grid[next_location.0 as usize][next_location.1 as usize] = true;
                    guards_current_position = next_location;
                }

                if match guards_current_direction {
                    '<' => guards_current_position.1 == 0,
                    '>' => guards_current_position.1 == (guard_map.grid[0].len() - 1) as i32,
                    'v' => guards_current_position.0 == (guard_map.grid.len() - 1) as i32,
                    '^' => guards_current_position.0 == 0,
                    _ => false,
                } {
                    break;
                }
            }

            guard_map.grid.iter().fold(i32::default(), |acc, row| {
                acc + row.iter().filter(|&&col| col).count() as i32
            })
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug, Default)]
/// A model of the puzzle input data
struct GuardMap {
    grid: Vec<Vec<bool>>,
    obstructions: Vec<(i32, i32)>,
    starting_position: (i32, i32),
}
impl GuardMap {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        puzzle_data.lines().enumerate().fold(
            GuardMap::default(),
            |mut guard_map, (line_idx, line)| {
                guard_map.grid.push(Vec::new());

                line.chars().enumerate().for_each(|(char_idx, char)| {
                    guard_map.grid[line_idx].push(false);

                    if char == '#' {
                        guard_map
                            .obstructions
                            .push((line_idx as i32, char_idx as i32));
                    } else if char == '^' {
                        guard_map.starting_position = (line_idx as i32, char_idx as i32);
                        guard_map.grid[line_idx][char_idx] = true;
                    }
                });

                guard_map
            },
        )
    }
}

