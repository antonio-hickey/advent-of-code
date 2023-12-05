/* Day - 4: Scratchcards */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");

    // Solve for part one and two
    println!("part one: {}", solution(&input, 1));
    println!("part two: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    match part {
        1 => {
            /* Solve for part one of the puzzle */
            // Calculate the points a card is worth, where every number won
            // after the first doubles in amount of points.
            input
                .lines()
                .map(|line| {
                    let scratchcard = Scratchcard::from_line(line);
                    let matching_numbers = scratchcard
                        .actual_numbers
                        .iter()
                        .filter(|&n| scratchcard.winning_numbers.contains(n))
                        .count();

                    if matching_numbers > 0 {
                        2i32.pow(matching_numbers as u32 - 1)
                    } else {
                        0
                    }
                })
                .sum()
        }
        2 => {
            /* Solve for part one of the puzzle */
            // Compute a multiplier effect based on how many numbers are won
            // for each scratchcard and then sum up the multipliers.
            let mut multiplier = vec![1; input.lines().count()];
            for (idx, line) in input.lines().enumerate() {
                let scratchcard = Scratchcard::from_line(line);
                let matching_numbers = scratchcard
                    .actual_numbers
                    .iter()
                    .filter(|&n| scratchcard.winning_numbers.contains(n))
                    .count();

                for i in (idx + 1)..(idx + 1 + matching_numbers) {
                    multiplier[i] += multiplier[idx];
                }
            }
            multiplier.iter().sum::<usize>() as i32
        }
        _ => panic!("there's only 2 parts brooooo"),
    }
}

/// A model of the scratchcard the elf has
struct Scratchcard {
    winning_numbers: Vec<i32>,
    actual_numbers: Vec<i32>,
}
impl Scratchcard {
    /// Parse a puzzle input line into a `Scratchcard`
    fn from_line(line: &str) -> Self {
        let scratchcard_parts: Vec<&str> = line.split(':').last().unwrap().split('|').collect();

        Self {
            winning_numbers: scratchcard_parts[0]
                .split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect(),
            actual_numbers: scratchcard_parts[1]
                .split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect(),
        }
    }
}
