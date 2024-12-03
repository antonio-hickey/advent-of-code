/* Day - 2: Red-Nosed Reports */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    let reactor_reports = ReactorReport::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            reactor_reports
                .into_iter()
                .filter(|report| {
                    let levels = report.levels.clone();

                    let is_increasing = levels[0] < levels[1];
                    for i in 0..levels.len() - 1 {
                        let delta = levels[i + 1] - levels[i];
                        if (is_increasing && delta < 1)
                            || (!is_increasing && delta > -1)
                            || delta.abs() > 3
                        {
                            return false;
                        }
                    }

                    true
                })
                .count() as i32
        }
        2 => {
            /* Part Two Solution */
            reactor_reports
                .into_iter()
                .filter(|report| {
                    let mut is_safe = true;
                    let levels = report.levels.clone();

                    let is_increasing = levels[0] < levels[1];
                    for i in 0..levels.len() - 1 {
                        let delta = levels[i + 1] - levels[i];
                        if (is_increasing && delta < 1)
                            || (!is_increasing && delta > -1)
                            || delta.abs() > 3
                        {
                            is_safe = false;
                            break;
                        }
                    }

                    if is_safe {
                        return true;
                    }

                    for i in 0..levels.len() {
                        let mut dampened_is_safe = true;
                        let mut dampened_levels = levels.to_vec();
                        dampened_levels.remove(i);

                        let is_increasing = dampened_levels[0] < dampened_levels[1];

                        for i in 0..dampened_levels.len() - 1 {
                            let delta = dampened_levels[i + 1] - dampened_levels[i];
                            if (is_increasing && delta < 1)
                                || (!is_increasing && delta > -1)
                                || delta.abs() > 3
                            {
                                dampened_is_safe = false;
                                break;
                            }
                        }

                        if dampened_is_safe {
                            return true;
                        }
                    }

                    false
                })
                .count() as i32
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A model of the puzzle input data
struct ReactorReport {
    levels: Vec<i64>,
}
impl ReactorReport {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Vec<Self> {
        puzzle_data
            .lines()
            .map(|line| {
                let levels: Vec<i64> = line
                    .split(" ")
                    .map(|level| level.parse().expect("AoC not to have bad data"))
                    .collect();
                ReactorReport { levels }
            })
            .collect()
    }
}
