/* Day - 3: Mull It Over */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i32 {
    let computer_memory = ComputerMemory::from_puzzle_input(input);
    let corrupted_memory = computer_memory.corrupted_memory;

    match part {
        1 => {
            /* Part One Solution */
            let mut valid_mul_ops: Vec<(i32, i32)> = Vec::new();
            let mut current_instruction_idx = 0;

            loop {
                let remaining_instructions = &corrupted_memory[current_instruction_idx..];
                if !remaining_instructions.contains("mul(") {
                    break;
                }

                let next_instruction_start = remaining_instructions
                    .find("mul")
                    .expect("AoC not to have bad data");
                let next_instruction_end = remaining_instructions[next_instruction_start..]
                    .find(')')
                    .expect("AoC not to have bad data")
                    + next_instruction_start;

                let possible_instruction =
                    &remaining_instructions[next_instruction_start..=next_instruction_end];

                if possible_instruction.len() >= 8 || possible_instruction.len() <= 12 {
                    if let Some(param_start) = possible_instruction.find('(') {
                        let param_end = possible_instruction.find(')').unwrap();
                        let param_space = &possible_instruction[param_start + 1..param_end];
                        if let Some(params_split) = param_space.split_once(',') {
                            if let (Ok(param_1), Ok(param_2)) =
                                (params_split.0.parse::<i32>(), params_split.1.parse::<i32>())
                            {
                                valid_mul_ops.push((param_1, param_2));
                                current_instruction_idx += next_instruction_end;
                                continue;
                            }
                        }
                    }
                }

                current_instruction_idx += next_instruction_start + 3;
            }

            valid_mul_ops
                .iter()
                .map(|mul_instruction| mul_instruction.0 * mul_instruction.1)
                .sum()
        }
        2 => {
            /* Part Two Solution: TODO: Clean this mess of a solution up */
            let mut valid_mul_ops: Vec<(i32, i32)> = Vec::new();
            let mut current_idx = 0;

            while current_idx < corrupted_memory.len() {
                let remaining_instructions = &corrupted_memory[current_idx..];

                let current_enabled_section_start = if current_idx == 0 {
                    0
                } else {
                    remaining_instructions
                        .find("do()")
                        .expect("AoC not to have bad data")
                        + current_idx
                };
                let current_enabled_section_end =
                    if let Some(enabled_section_end) = remaining_instructions.find("don't()") {
                        enabled_section_end + current_idx
                    } else {
                        corrupted_memory.len()
                    };

                if current_enabled_section_end < current_enabled_section_start {
                    current_idx = current_enabled_section_end + 7;
                    continue;
                }

                let enabled_section =
                    &corrupted_memory[current_enabled_section_start..current_enabled_section_end];

                let mut current_instruction_idx = 0;
                while current_instruction_idx < enabled_section.len() {
                    let remaining_instructions = &enabled_section[current_instruction_idx..];
                    if !remaining_instructions.contains("mul(") {
                        break;
                    }

                    let next_instruction_start = remaining_instructions
                        .find("mul")
                        .expect("AoC not to have bad data");
                    let next_instruction_end = remaining_instructions[next_instruction_start..]
                        .find(')')
                        .expect("AoC not to have bad data")
                        + next_instruction_start;

                    let possible_instruction =
                        &remaining_instructions[next_instruction_start..=next_instruction_end];

                    if possible_instruction.len() >= 8 || possible_instruction.len() <= 12 {
                        if let Some(param_start) = possible_instruction.find('(') {
                            let param_end = possible_instruction.find(')').unwrap();
                            let param_space = &possible_instruction[param_start + 1..param_end];
                            if let Some(params_split) = param_space.split_once(',') {
                                if let (Ok(param_1), Ok(param_2)) =
                                    (params_split.0.parse::<i32>(), params_split.1.parse::<i32>())
                                {
                                    valid_mul_ops.push((param_1, param_2));
                                    current_instruction_idx += next_instruction_end;
                                    continue;
                                }
                            }
                        }
                    }

                    current_instruction_idx += next_instruction_start + 3;
                }

                current_idx = current_enabled_section_end + 7;
            }

            valid_mul_ops
                .iter()
                .map(|mul_instruction| mul_instruction.0 * mul_instruction.1)
                .sum()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug, Default)]
/// A model of the puzzle input data
struct ComputerMemory {
    corrupted_memory: String,
}
impl ComputerMemory {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        ComputerMemory {
            corrupted_memory: puzzle_data.to_string(),
        }
    }
}
