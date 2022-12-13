use std::collections::HashSet;

#[derive(Debug, Default)]
struct Map {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

fn main() {
    let input = include_str!("./input.prod").lines().collect::<Vec<&str>>();
    let x = input[0].len() as i32;
    let y = input.len() as i32;

    let mut trees = Map {
        grid: Vec::new(),
        width: x,
        height: y,
    };

    for line_of_trees in input.iter() {
        trees.grid.push(line_of_trees.chars().collect::<Vec<char>>());
    }

    println!("Part One Answer: {:?}", solution(trees));
}

fn solution(tree_map: Map) -> usize {
    const NORTH: (i32, i32) = (-1, 0);
    const SOUTH: (i32, i32) = (1, 0);
    const EAST: (i32, i32) = (0, -1);
    const WEST: (i32, i32) = (0, 1);

    let mut visable_trees = HashSet::new();

    for (start, step, search) in [
        ((0, 0), WEST, SOUTH),
        ((0, 0), SOUTH, WEST),
        ((tree_map.height - 1, tree_map.width - 1), NORTH, EAST),
        ((tree_map.height - 1, tree_map.width - 1), EAST, NORTH),
    ] {
        let mut walk = start;

        while walk.0 >= 0 && walk.0 < tree_map.height && walk.1 >= 0 && walk.1 < tree_map.width {
            let (mut row, mut col) = walk;
            let mut tallest_tree = tree_map.grid[row as usize][col as usize];

            visable_trees.insert((row, col));

            while tallest_tree < '9' {
                row += search.0;
                col += search.1;

                if row < 0 || row >= tree_map.height || col < 0 || col >= tree_map.width {
                    break;
                }

                let tree = tree_map.grid[row as usize][col as usize];
                if tree > tallest_tree {
                    visable_trees.insert((row, col));
                    tallest_tree = tree;
                }
            }

            walk.0 += step.0;
            walk.1 += step.1;
        }
    }

    return visable_trees.len();
}

