use std::fs;

fn solution() -> usize {
    let input = fs::read_to_string("./input.txt").expect("uh oh");
    let t: Vec<&str> = input.split("\n\n").collect();

    let mut highest_cals: i32 = 0;
    let mut elve_id_with_most_cals: usize = 0;

    for (elve_id, food_stash) in t.iter().enumerate() {
        let food_items: Vec<&str> = food_stash.split("\n").filter(|&x| !x.is_empty()).collect();
        let mut _sum: i32 = 0;

        for food_item in food_items {
            let calories: i32 = food_item.parse::<i32>().unwrap();
            _sum += calories;
        }

        if _sum > highest_cals {
            highest_cals = _sum;
            elve_id_with_most_cals = elve_id;
        }
    }

    return elve_id_with_most_cals;
}

fn main() {
    let answer: usize = solution();
    println!("Answer is: {}", answer);
}
