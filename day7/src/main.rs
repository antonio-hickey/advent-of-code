use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Directory {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    subdir: RefCell<HashMap<String, Rc<Directory>>>,
}
impl Directory {
    fn get_size(&self) -> usize {
        *self.size.borrow() + self
            .subdir.borrow().values()
            .fold(0, |x, y| x + y.get_size())
    }
}

fn main() {
    // Ewww this problem was hard in Rust, but learnt a lot about refrences :)

    let input = include_str!("./input.prod");

    println!("Part One Answer: {}", solution(input, 1));
    println!("Part Two Answer: {}", solution(input, 2));
}

fn solution(input: &str, part: i8) -> usize {
    let root_directory = Rc::new(Directory {
        _name: String::from("/"),
        size: RefCell::new(0),
        parent: None,
        subdir: RefCell::new(HashMap::new()),
    });
    let mut cwd = Rc::clone(&root_directory);

    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();

        // Pretty cool way to handle conditions in rust
        match (words[0], words[1]) {
            ("$", "ls") => {},
            ("$", "cd") => {
                match words[2] {
                    "/" => cwd = Rc::clone(&root_directory),
                    ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                    dirname => {
                        let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    }
                }
            },
            ("dir", directory_name) => {
                cwd.subdir.borrow_mut().insert(
                    String::from(directory_name),
                    Rc::new(Directory { 
                        _name: String::from(directory_name), 
                        size: RefCell::new(0), 
                        parent: Some(Rc::clone(&cwd)), 
                        subdir: RefCell::new(HashMap::new()),
                    }),
                );
            },
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    // Part two variables
    let total_space = &root_directory.get_size();
    let free_space = 70_000_000 - total_space;
    let space_needed = 30_000_000 - free_space;

    let mut to_visit = vec![Rc::clone(&root_directory)];

    // Initialize Part one output 
    // (total size of all directories under or equal to 100,000)
    let mut total = 0;

    // Part two output 
    // (the smallest directory that is greater or equal to space_needed)
    let mut best = usize::MAX;


    while let Some(directory) = to_visit.pop() {
        to_visit.extend(directory.subdir.borrow().values().map(Rc::clone));
        let size = &directory.get_size();

        // Part one solution
        if part == 1 && size <= &usize::try_from(100_000).unwrap() {
            total += size;
        }

        // Part two solution
        if part == 2 && size >= &space_needed {
            best = best.min(*size);
        }
    }

    if part == 1 {
        return total
    } else {
        return best
    }
}
