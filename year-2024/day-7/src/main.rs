/* Day 7: Bridge Repair */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    let calibrations = Calibration::from_puzzle_input(input);

    // parse the puzzle input into a useful model
    match part {
        1 => {
            /* Part One Solution */
            calibrations
                .iter()
                .filter(|calibration| is_valid_calibration(calibration))
                .map(|calibration| calibration.target)
                .sum()
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

/// Determine if a `Calibration`
fn is_valid_calibration(calibration: &Calibration) -> bool {
    let mut is_valid_calibration = false;
    let params = &calibration.parameters;

    let mut equation_results = vec![params[0] as i64];
    params[1..].iter().for_each(|&param| {
        let mut next_results = Vec::new();

        equation_results.iter().for_each(|result| {
            next_results.push(result + param as i64);
            next_results.push(result * param as i64);
        });

        equation_results = next_results;
    });

    if equation_results.contains(&calibration.target) {
        is_valid_calibration = true;
    }

    is_valid_calibration
}

#[derive(Debug, Default)]
/// A model of the puzzle input data
struct Calibration {
    target: i64,
    parameters: Vec<i32>,
}
impl Calibration {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Vec<Self> {
        puzzle_data
            .lines()
            .fold(Vec::new(), |mut calibrations, line| {
                let mut sections = line.split(":");

                let target: i64 = sections
                    .next()
                    .expect("AoC not to have bad data")
                    .parse()
                    .expect("AoC not to have bad data");

                let parameters: Vec<i32> = sections
                    .next()
                    .expect("AoC not to have bad data")
                    .trim_start()
                    .split(" ")
                    .map(|param| param.trim().parse().expect("AoC not to have bad data"))
                    .collect();

                calibrations.push(Calibration { target, parameters });
                calibrations
            })
    }
}
