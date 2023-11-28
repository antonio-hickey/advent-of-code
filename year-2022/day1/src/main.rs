use std::fs;

fn shift_vector(vec: &mut Vec<i32>, idx: usize, val: i32) -> &mut Vec<i32> {
    if idx == 0 {
        vec[2] = vec[1];
        vec[1] = vec[0];
        vec[0] = val;
    } else if idx == 1 {
        vec[2] = vec[1];
        vec[1] = val;
    } else {
        vec[2] = val;
    }

    return vec;
}

fn solution() -> Vec<i32> {
    let input = fs::read_to_string("./input.txt").expect("uh oh");
    let elves: Vec<&str> = input.split("\n\n").collect();

    let mut top_3: Vec<i32> = Vec::from([0, 0, 0]);

    // TODO: can def do this with less loops, but advent of code is scored
    // on how fast you can come up with a solution not how fast your solution's
    // runtime is so we ignoooore for now :)
    for elve in elves {
        let food_items: Vec<&str> = elve.split("\n").filter(|&x| !x.is_empty()).collect();
        let mut _sum: i32 = 0;

        for food_item in food_items {
            let calories: i32 = food_item.parse::<i32>().unwrap();
            _sum += calories;
        }

        let mut new_top_3 = top_3.clone();
        for (idx, &mut val) in top_3.iter_mut().enumerate() {
            if _sum > val {
                shift_vector(&mut new_top_3, idx, _sum);
                break;
            }
        }

        top_3 = new_top_3;
    }

    return top_3;
}

fn main() {
    let partial_solution = solution();

    let part1_answer = partial_solution[0];
    println!("Part 1 answer is: {}", part1_answer);

    let part2_answer: i32 = partial_solution.iter().sum();
    println!("Part 2 answer is: {}", part2_answer);
}
