/* Day 3 - Gear Ratios */

use std::collections::HashSet;
use std::fs;

fn main() {
    // Read puzzle input data into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");

    // Solve for part one
    let part_one = solution(&input, 1);
    println!("{part_one}");

    // Solve for part two
    let part_two = solution(&input, 2);
    println!("{part_two}");
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    // Initiate a mutable representation of the EngineSchematic to ylect
    // coordinates of symbols and part numbers as well as their values
    let mut engine_schematic = EngineSchematic::default();

    // Parse the puzzle input into `EngineSchematic`
    let mut current_number: Option<PartNumber> = None;
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            // Check each character in the engine schematic (puzzle input)
            // to see if it's a digit or symbol
            if char.is_ascii_digit() {
                // It's a number
                let char_number = char.to_digit(10).unwrap() as i32;
                if let Some(ref mut number) = current_number {
                    // `PartNumber` already instantiated
                    number.append_digit(char_number);
                    number.extend_coords(x as i32, y as i32);
                } else {
                    // Create a new `PartNumber` instance
                    current_number = Some(PartNumber::new(char_number, x as i32, y as i32));
                }
            } else {
                // It's either a parsed `PartNumber`, a symbol, or '.'
                if let Some(number) = current_number.take() {
                    // The character is part_number so push it to engine schematic
                    // part numbers vector
                    engine_schematic.part_numbers.push(number);
                }
                if char != '.' {
                    // The character is a symbol, so insert a new entry in
                    // engine schematic symbols hashset with it's coordinates (x, y)
                    engine_schematic.symbols.insert((x as i32, y as i32));
                    if char == '*' {
                        engine_schematic.gears.insert((x as i32, y as i32));
                    }
                }
            }
        }
    }

    match part {
        1 => {
            // * Solve for part one of the puzzle *
            // Iterate over each possible part number in the engine schematic
            // and filter out the ones that are not adjacent to a symbol by
            // intersecting number coordinates with the hash-set of symbol
            // coordinates. Finally summing up all the symbol adjacent part numbers.
            engine_schematic
                .part_numbers
                .iter()
                .filter(|num| {
                    num.adjacent_coords
                        .intersection(&engine_schematic.symbols)
                        .next()
                        .is_some()
                })
                .map(|num| num.number)
                .sum::<i32>()
        }
        2 => {
            // * Solve for part two of the puzzle *
            // Iterate over the gears in the engine schematic filtering for only gears
            // with 2 part numbers adjacent to it and calculating the product of the 2
            // adjacent numbers and then finally summing up all the products.
            engine_schematic
                .gears
                .iter()
                .filter_map(|gear| {
                    let matches: Vec<i32> = engine_schematic
                        .part_numbers
                        .iter()
                        .filter(|num| num.adjacent_coords.contains(gear))
                        .take(2)
                        .map(|num| num.number)
                        .collect();

                    if matches.len() == 2 {
                        Some(matches[0] * matches[1])
                    } else {
                        None
                    }
                })
                .sum()
        }
        _ => panic!("only 2 parts brooooo"),
    }
}

/// A representation of the engine schematic data
/// seperating the part numbers from the symbols.
#[derive(Default, Debug)]
struct EngineSchematic {
    part_numbers: Vec<PartNumber>,
    symbols: HashSet<(i32, i32)>,
    gears: HashSet<(i32, i32)>,
}

/// A representation of the part numbers in the engine
/// schematic data ylecting the number and the adjacent points
/// around it in the schematic.
#[derive(Debug)]
struct PartNumber {
    pub number: i32,
    adjacent_coords: HashSet<(i32, i32)>,
}
impl PartNumber {
    /// Creates a new instance of `PartNumber`
    fn new(char: i32, x: i32, y: i32) -> Self {
        let coordinates = HashSet::from([
            // Coordinates left of the part number
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            // Coordinates below and above the part number
            (x - 1, y),
            (x + 1, y),
            // Coordinates right of the part number
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]);

        Self {
            number: char.to_string().parse().unwrap(),
            adjacent_coords: coordinates,
        }
    }

    /// Appends a new digit to a `PartNumber` instance
    fn append_digit(&mut self, digit: i32) {
        self.number = self.number * 10 + digit
    }

    /// Extends the coordinates hashset in a `PartNumber` instance
    fn extend_coords(&mut self, x: i32, y: i32) {
        self.adjacent_coords
            .extend([(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)])
    }
}
