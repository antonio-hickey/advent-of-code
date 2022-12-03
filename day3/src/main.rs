mod items;
pub use items::item_priorities;


fn main() {
    let priorities = item_priorities();
    let priority_of_items_in_both: Vec<i32> = include_str!("./input.test")
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

    println!("Part 1 answer: {:?}", priority_of_items_in_both.iter().sum::<i32>());
}

