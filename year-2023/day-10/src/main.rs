/* Day - 10: Pipe Maze */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    // parse the puzzle input into a useful model
    let maze = Maze::from_puzzle_input(input);
    println!("maze: {maze:?}");

    match part {
        1 => {
            /* Part One Solution */
            todo!()
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A model of the puzzle input data
struct Maze {
    starting_point: (i64, i64),
    tile_map: Vec<Vec<Tile>>,
}
impl Maze {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        // Iterate over each line in the puzzle data while keeping track
        // of what line number your on, and then iterate over each character
        // in the line while keeping track of the character number your on.
        let mut starting_point = (0, 0);
        let tile_map = puzzle_data
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, ch)| {
                        // Figure out what type of tile it is
                        let tile_type = match ch {
                            '|' => TileType::VerticalPipe,
                            '-' => TileType::HorizontalPipe,
                            'L' => TileType::NorthToEastBend,
                            'J' => TileType::NorthToWestBend,
                            '7' => TileType::SouthToWestBend,
                            'F' => TileType::SouthToEastBend,
                            '.' => TileType::Ground,
                            'S' => {
                                starting_point = (x as i64, y as i64);
                                TileType::StartingPoint
                            }
                            _ => panic!("unaccounted for TileType"),
                        };

                        Tile {
                            coordinates: (x as i64, y as i64),
                            value: tile_type,
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            starting_point,
            tile_map,
        }
    }
}

#[derive(Debug)]
/// A model to represent the tiles in the maze
struct Tile {
    /// (x, y) coordinates of the tile in the maze(2-D)
    coordinates: (i64, i64),
    /// The type of tile it is
    value: TileType,
}

#[derive(Debug)]
/// A model to represent the different types
/// of tiles in the maze
enum TileType {
    VerticalPipe,
    HorizontalPipe,
    NorthToEastBend,
    NorthToWestBend,
    SouthToWestBend,
    SouthToEastBend,
    Ground,
    StartingPoint,
}
