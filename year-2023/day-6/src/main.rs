/* Day - 6: Wait For It */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    // parse the puzzle input into a useful model
    let boat_race_competition = BoatRaceCompetition::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            // Iterate over each race, Iterate over each millisecond (time step) in the race
            // to compute each possible strategy I can take to maximize distance and then filter
            // them out by checking if it's greater than the distance to beat for the ith race.
            // Then take a sum of all the possible winning strategies I can take in each race
            // and finally compute a product from it.
            boat_race_competition
                .races
                .iter()
                .map(|race| {
                    (0..race.duration + 1)
                        .filter_map(|time_step| {
                            if (race.duration - time_step) * time_step > race.distance_to_beat {
                                return Some(1);
                            }
                            None
                        })
                        .sum::<i64>()
                })
                .product()
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A useful model of the puzzle input data
struct BoatRaceCompetition {
    races: Vec<Race>,
}
impl BoatRaceCompetition {
    /// Parse the puzzle input data into a useful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        let mut time_values: Vec<i64> = Vec::new();
        let mut distance_values: Vec<i64> = Vec::new();
        for (idx, line) in puzzle_data.lines().enumerate() {
            let line_split_up = line.split(':').last().unwrap().split(' ');
            if idx == 0 {
                time_values = line_split_up
                    .filter_map(|time_value| time_value.parse::<i64>().ok())
                    .collect();
            } else {
                distance_values = line_split_up
                    .filter_map(|distance_value| distance_value.parse::<i64>().ok())
                    .collect();
            }
        }

        Self {
            races: time_values
                .iter()
                .enumerate()
                .map(|(idx, time_val)| Race {
                    duration: time_val.to_owned(),
                    distance_to_beat: distance_values[idx],
                })
                .collect(),
        }
    }
}

#[derive(Debug, Default)]
/// A useful model of each individual race in the puzzle input
struct Race {
    duration: i64,
    distance_to_beat: i64,
}
