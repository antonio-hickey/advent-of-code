/* Day - 4: Scratchcards */

use std::{collections::HashSet, fs};

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");

    // Solve for part one
    let part_one = solution(&input, 1);
    println!("{part_one}");
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    input
        .lines()
        .map(|line| {
            // Each scratchcard defaults to 0 points
            let mut card_points: i32 = 0;

            // Split the line (scratchcard) into 2 parts (1 for winning numbers, and another for actual numbers)
            let card_parts: Vec<&str> = line.split(':').last().unwrap().split('|').collect();
            if let (Some(winning_numbers_part), Some(actual_numbers_part)) =
                (card_parts.get(0), card_parts.get(1))
            {
                // These are the numbers that will win for the scratchcard
                let winning_numbers: Vec<i32> = winning_numbers_part
                    .split_whitespace()
                    .filter_map(|number| number.parse().ok())
                    .collect();

                // These are the actual numbers are scratchcard has
                let actual_numbers: Vec<i32> = actual_numbers_part
                    .split_whitespace()
                    .filter_map(|number| number.parse().ok())
                    .collect();

                // These are the actual numbers our card has that match a winning number
                let numbers_won: Vec<i32> = actual_numbers
                    .iter()
                    .filter(|&number| winning_numbers.contains(number))
                    .map(|&number| number)
                    .collect();

                match part {
                    1 => {
                        /* Solve for part one of the puzzle */
                        // Calculate the points a card is worth, where every number won
                        // after the first doubles in amount of points.
                        if !numbers_won.is_empty() {
                            card_points = 1;

                            // Handle the doubling of points if more than 1 number won
                            if numbers_won.len() > 1 {
                                card_points *= 2i32.pow((numbers_won.len() - 1) as u32);
                            }
                        }
                    }
                    2 => todo!(),
                    _ => panic!("there's only 2 parts brooooo"),
                }
            }
            card_points
        })
        .sum()
}
