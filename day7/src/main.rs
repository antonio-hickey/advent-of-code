use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Directory {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    subdir: RefCell<HashMap<String, Rc<Directory>>>,
}

fn main() {
    // Ewww this problem was hard in Rust, but learnt a lot about refrences :)

    let input = include_str!("./input.test");

    let root_directory = Rc::new(Directory {
        name: String::from("/"),
        size: RefCell::new(0),
        parent: None,
        subdir: RefCell::new(HashMap::new()),
    });

    println!("Part One Answer: {}", solution(input, &root_directory));
}

fn get_size(dir: &Rc<Directory>) -> usize {
    *dir.size.borrow() + dir.subdir.borrow().values().fold(0, |a, b| a + get_size(b))
}

fn solution(input: &str, root_directory: &Rc<Directory>) -> usize {
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
                        name: String::from(directory_name), 
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

    let mut to_visit = vec![Rc::clone(&root_directory)];
    let mut total = 0;

    while let Some(directory) = to_visit.pop() {
        to_visit.extend(directory.subdir.borrow().values().map(Rc::clone));

        let size = get_size(&directory);
        if size <= 100000 {
            total += size;
        }
    }

    return total;
}
