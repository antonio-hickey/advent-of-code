/* Day - 5: If You Give A Seed A Fertilizer */

use std::{fs, ops::Range};

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");

    println!("part one solution: {}", solution(&input, 1));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    let almanac = Almanac::from_input_str(input);

    match part {
        1 => {
            /* Part One Solution */
            // Iterate over each seed then recurcively search the
            // maps for a location, while keeping track of the
            // shortest distance location (minimum of locations)
            almanac
                .seeds
                .iter()
                .map(|seed| {
                    almanac
                        .maps
                        .iter()
                        .fold(*seed, |acc, map| map.map_lookup(acc))
                })
                .min()
                .unwrap_or(i64::MAX)
        }
        2 => todo!(),
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

/// A model of the puzzle input data
#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Mappings>,
}
impl Almanac {
    /// Parse the puzzle input into a instance of Almanac
    // This puzzles parsing was annoying affff lol
    fn from_input_str(input: &str) -> Self {
        // Puzzle input as lines of text
        let lines: Vec<&str> = input.lines().collect();

        // Initialize struct values
        let seeds = lines[0]
            .replace("seeds: ", "")
            .split(' ')
            .map(|number| number.parse().unwrap())
            .collect();

        // Initialize a mutable Almanac instance
        let mut self_ = Self {
            seeds,
            maps: Vec::new(),
        };

        let mut current_map = Mappings::default();
        for line in lines[2..].iter() {
            if line == &"" {
                self_.maps.push(current_map);
                current_map = Mappings::default();
                continue;
            }
            if line.contains(':') {
                self_.maps.push(current_map);
                current_map = Mappings::default();
                continue;
            }

            let nums: Vec<i64> = line
                .trim()
                .split(' ')
                .map(|number| number.parse().unwrap())
                .collect();

            current_map.add_mapping(nums[0], nums[1], nums[2]);
        }
        if !current_map.maps.is_empty() {
            self_.maps.push(current_map);
        }

        self_
    }
}

#[derive(Debug, Default, PartialEq)]
struct Mapping {
    range: Range<i64>,
    delta: i64,
}

#[derive(Debug, Default, PartialEq)]
struct Mappings {
    maps: Vec<Mapping>,
}
impl Mappings {
    /// Add a parsed mapping to the Almanac and sort the maps by range start
    fn add_mapping(&mut self, destination: i64, source: i64, length: i64) {
        self.maps.push(Mapping {
            range: Range {
                start: source,
                end: source + length,
            },
            delta: destination - source,
        });
        self.maps.sort_by_key(|mapping| mapping.range.start)
    }

    /// Use the map to look up a value and get it's location
    /// Returns a None in the case of no maps else Some value
    fn map_lookup(&self, val: i64) -> i64 {
        for map in &self.maps {
            if map.range.contains(&val) {
                return val + map.delta;
            }
        }

        val
    }
}
