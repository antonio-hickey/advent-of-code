/* Day 1 - Trebuchet?! */

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
    input
        .lines()
        .map(|l| {
            if part.eq(&1) {
                // * Solve for part one of the puzzle *
                // Just going character by character and
                // filtering for and mapping to digits.
                l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
            } else {
                // * Solve for part two of the puzzle *
                // Same as part one solution, but replace any
                // spelt out digits with the spelt out + digit + spelt out
                l.to_string()
                    .replace("zero", "zero0zero")
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
            }
        })
        .map(|digits| {
            if !digits.is_empty() {
                // if the vector of digits is not empty then concatenate the first and last digits in the vector
                // for ex: [2, 3, 4] -> 2 * 10 = 20 + 4 = 24
                10 * digits.first().expect("1 digit or more")
                    + digits.last().expect("1 digit or more")
            } else {
                0
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
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two() {
        let input_test_data =
            fs::read_to_string("./input_test_part_two.txt").expect("some input test data");
        let result = solution(&input_test_data, 2);
        assert_eq!(result, 281);
    }
}
