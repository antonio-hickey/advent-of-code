/* Day - 9: Mirage Maintenance */

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
    let oasis_report = OasisReport::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            // Iterate over each historic point (line) in the sand instability sensor
            // then iterate over each value in the line and aggregate a sequence of
            // differences between each pair of values. Using it to predict the next
            // value for this historic point (line), and finally summing up all the
            // next values to get the puzzle answer.
            oasis_report
                .sensor_history
                .iter()
                .map(|nums| {
                    let mut prediction = *nums.last().unwrap();
                    let mut differences: Vec<i64> =
                        nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
                    prediction += differences.last().unwrap();

                    loop {
                        differences = differences
                            .windows(2)
                            .map(|pair| pair[1] - pair[0])
                            .collect();
                        if differences[0] == differences[1]
                            && differences.iter().all(|num| *num == 0)
                        {
                            return prediction;
                        }
                        prediction += differences.last().unwrap();
                    }
                })
                .sum()
        }
        2 => {
            /* Part Two Solution */
            // Same as part one but reverse the nums in a sensor history line
            // before computing a prediction based on difference sequences.
            oasis_report
                .sensor_history
                .iter()
                .map(|nums| {
                    let nums = nums.iter().rev().copied().collect::<Vec<i64>>();
                    let mut prediction = *nums.last().unwrap();
                    let mut differences: Vec<i64> =
                        nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
                    prediction += differences.last().unwrap();

                    loop {
                        differences = differences
                            .windows(2)
                            .map(|pair| pair[1] - pair[0])
                            .collect();
                        if differences[0] == differences[1]
                            && differences.iter().all(|num| *num == 0)
                        {
                            return prediction;
                        }
                        prediction += differences.last().unwrap();
                    }
                })
                .sum()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A model of the puzzle input data
struct OasisReport {
    sensor_history: Vec<Vec<i64>>,
}
impl OasisReport {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        Self {
            // Go over each line in the puzzle data
            // and split up each character in the line
            // mapping them into an integer
            sensor_history: puzzle_data
                .lines()
                .map(|line| {
                    line.split(' ')
                        .map(|number| number.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}
