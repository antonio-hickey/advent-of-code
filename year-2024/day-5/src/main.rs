/* Day - 5: Print Queue */

use std::collections::HashMap;
use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    let safety_manual = SafetyManual::from_puzzle_input(input);

    // parse the puzzle input into a useful model
    match part {
        1 => {
            /* Part One Solution */
            safety_manual
                .updates
                .iter()
                .fold(i32::default(), |acc, update| {
                    let mut index_map = HashMap::new();
                    update.iter().enumerate().for_each(|(idx, page)| {
                        index_map.insert(page, idx);
                    });

                    let mut is_correct = true;
                    safety_manual.ordering_rules.iter().for_each(|(x, ys)| {
                        if let Some(&x_idx) = index_map.get(x) {
                            for y in ys {
                                if let Some(y_idx) = index_map.get(y) {
                                    if x_idx > *y_idx {
                                        is_correct = false;
                                    }
                                }
                            }
                        }
                    });

                    if is_correct {
                        acc + update[update.len() / 2]
                    } else {
                        acc
                    }
                })
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A model of the puzzle input data
struct SafetyManual {
    ordering_rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}
impl SafetyManual {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut updates: Vec<Vec<i32>> = Vec::new();
        let sections: Vec<&str> = puzzle_data.split("\n\n").collect();

        let ordering_rules_section = sections[0];
        let updates_section = sections[1];

        ordering_rules_section.lines().for_each(|line| {
            let line_split: Vec<&str> = line.split("|").collect();

            let left: i32 = line_split[0].parse().expect("AoC not to have bad data");
            let right: i32 = line_split[1].parse().expect("AoC not to have bad data");

            ordering_rules.entry(left).or_default().push(right)
        });

        updates_section.lines().enumerate().for_each(|(idx, line)| {
            updates.push(Vec::new());
            line.split(",").for_each(|page| {
                updates[idx].push(page.parse().expect("AoC not to have bad data"))
            });
        });

        SafetyManual {
            ordering_rules,
            updates,
        }
    }
}
