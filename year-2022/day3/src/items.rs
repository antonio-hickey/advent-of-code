use std::collections::HashMap;

pub fn item_priorities() -> HashMap<String, usize> {
    let items: Vec<String> = Vec::from([
        String::from("a"), String::from("b"), String::from("c"), 
        String::from("d"), String::from("e"), String::from("f"), 
        String::from("g"), String::from("h"), String::from("i"), 
        String::from("j"), String::from("k"), String::from("l"), 
        String::from("m"), String::from("n"), String::from("o"), 
        String::from("p"), String::from("q"), String::from("r"), 
        String::from("s"), String::from("t"), String::from("u"), 
        String::from("v"), String::from("w"), String::from("x"), 
        String::from("y"), String::from("z"),
    ]);

    let mut item_priority = HashMap::<String, usize>::new();
    for (val, key) in items.iter().enumerate() { 
        let priority = val + 1;
        item_priority.insert(String::from(key), priority);

        // insert the CAPITAL version of the item where the priority
        // is the length of vector plus the iteration.
        item_priority.insert(
            String::from(key.to_uppercase()), 
            items.len() + priority,
        );
    }

    return item_priority
}
