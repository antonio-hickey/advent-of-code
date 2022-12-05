// TODO: 
//   - NEEDS BIG REFACTOR, HAD TROUBLE PARSING SO SUPER HACKY
//     PARSING TO GET ANSWER ASAP.

fn main() {
    println!("Part 1 Answer: {}", solution(1));
}

fn solution(part: i8) -> String {
    let (_crates, _instructions) = include_str!("./input.prod").split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<String>> = Vec::new();

    for line in _crates.lines().filter(|line| !line.is_empty()) {
        let stack_i = line.chars().collect::<Vec<char>>().chunks(4)
            .map(|c| c.iter().collect::<String>().replace(" ", ""))
            .collect::<Vec<String>>();

        if stacks.is_empty() {
            for _ in 0..stack_i.len() { stacks.push(Vec::new()) }
        }

        for (idx, val) in stack_i.iter().enumerate() {
            if val.contains("[") {
                stacks[idx].push(val.clone());
            }
        }
    }


    for t in _instructions.lines() {
        let instructions_split = t.split(" ").collect::<Vec<&str>>();

        let n_crates = instructions_split[1].parse::<i32>().unwrap();
        let from_stack_id = instructions_split[3].parse::<i32>().unwrap() - 1;
        let to_stack_id = instructions_split[5].parse::<i32>().unwrap() - 1;


        for _ in 0..n_crates {
            let mut crate_to_move = String::new();
            if stacks[from_stack_id as usize].len() == 1 {
                crate_to_move = stacks[from_stack_id as usize].pop().unwrap();
            } else {
                crate_to_move = stacks[from_stack_id as usize].remove(0 as usize);
            }

            stacks[to_stack_id as usize].insert(0 as usize, crate_to_move);
        }
    }

    let mut answer = String::new();

    for stack in stacks {
        answer = answer + &stack.first().unwrap().replace("[", "").replace("]", "");
    }

    return answer;
}

