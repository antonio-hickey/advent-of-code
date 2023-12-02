/* Day 2 - Cube Conundrum */

use std::collections::HashMap;
use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./input.txt").expect("some puzzle input data");

    // Solve for part one
    let part_one = solution(&input, 1);
    println!("{part_one}");

    // Solve for part two
    let part_two = solution(&input, 2);
    println!("{part_two}");
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: u8) -> u32 {
    // The known amount of cubes by color
    let known_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    // Parse through the puzzle input and run the logic to solve it
    input
        .lines()
        .filter_map(|l| {
            // Parse out the line into meaningful parts
            let mut parts = l.split(':');
            let game_id_part = parts.next().unwrap();
            let game_data_part = parts.next().unwrap();

            // Parse out the game id
            let game_id: u32 = game_id_part.replace("Game", "").trim().parse().unwrap();

            if part.eq(&1) {
                // * Solve for part one of the puzzle *
                // Check if each game is possible if it is add the
                // game id to sum up with other possible games id's
                let is_possible = game_data_part.split(';').all(|subset| {
                    subset
                        .split(',')
                        .map(|cubes| cubes.trim().split_whitespace().collect::<Vec<&str>>())
                        .all(|n_cubes_and_color| {
                            let n_cubes: u32 = n_cubes_and_color[0].parse().unwrap();
                            let color = n_cubes_and_color[1];
                            n_cubes <= known_cubes[color]
                        })
                });

                if is_possible {
                    Some(game_id)
                } else {
                    None
                }
            } else {
                // * Solve for part two of the puzzle *
                // Filter out impossible games based on the known cubes
                // then find out the minimum number of cubes for each color
                // for the game to have been played, then multiply them
                // together and finally summing all the products

                let mut minimum_cubes: HashMap<&str, u32> =
                    HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

                game_data_part.split(';').all(|subset| {
                    subset
                        .split(',')
                        .map(|cubes| cubes.trim().split_whitespace().collect::<Vec<&str>>())
                        .all(|n_cubes_and_color| {
                            // Parse out the number of cubes and their color
                            let n_cubes: u32 = n_cubes_and_color[0].parse().unwrap();
                            let color = n_cubes_and_color[1];

                            // Set the minimum needed cubes by color
                            if n_cubes > minimum_cubes[color] {
                                let entry = minimum_cubes.entry(color).or_insert(0);
                                *entry = (*entry).max(n_cubes);
                            }

                            true
                        })
                });

                // Multiply each value together
                Some(minimum_cubes.values().product())
            }
        })
        .sum()
}

#[cfg(test)]
/// Tests
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_test_data =
            fs::read_to_string("./input_test_part_one.txt").expect("some input test data");
        let result = solution(&input_test_data, 1);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two() {
        let input_test_data =
            fs::read_to_string("./input_test_part_two.txt").expect("some input test data");
        let result = solution(&input_test_data, 2);
        assert_eq!(result, 2286);
    }
}
