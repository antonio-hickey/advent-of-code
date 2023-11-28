use std::collections::HashMap;

mod items;
pub use items::item_priorities;


fn main() {
    let priorities = item_priorities();

    let part_1_solution = part_one_solution(&priorities);
    println!("Part 1 answer: {:?}", part_1_solution);

    let part_2_solution = part_two_solution(&priorities);
    println!("Part 2 answer: {:?}", part_2_solution);
}


fn part_one_solution(priorities: &HashMap<String, usize>) ->i32 {
    let priority_of_items_in_both: Vec<i32> = include_str!("./input.prod")
        .split("\n")
        .map(|i| {
            let (compartment_one, compartment_two) = i.split_at(i.len() / 2);

            let mut in_both: Vec<usize> = Vec::new();
            for item in compartment_one.chars() {
                let item_string = String::from(item);
                if compartment_two.contains(&item_string) {
                    let priority = priorities[&item_string];
                    if !in_both.contains(&priority) {
                        in_both.push(priority);
                    }
                }
            }
            return in_both.iter().map(|x| *x as i32).sum();
        })
        .collect::<Vec<i32>>();
    
    priority_of_items_in_both.iter().sum::<i32>()
} 


fn part_two_solution(priorities: &HashMap<String, usize>) -> i32 {
    let x = include_str!("./input.prod")
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|elve_group| {
            let mut badge_priority: i32 = 0;
            for item in elve_group[0].chars() {
                if elve_group[1].contains(item) && elve_group[2].contains(item) {
                    badge_priority = priorities[&String::from(item)] as i32;
                }
            }
            return badge_priority;
        })
        .collect::<Vec<i32>>();

    x.iter().sum::<i32>()
}

