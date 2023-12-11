/* Day - 10: Pipe Maze */

use std::collections::HashMap;
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
    //println!("maze: {maze:?}");

    match part {
        1 => {
            /* Part One Solution */
            let start_tile = maze
                .tile_map
                .get(maze.starting_point.0 as usize)
                .unwrap()
                .get(maze.starting_point.1 as usize)
                .unwrap();

            for connecting_adjacent_tile in start_tile
                .get_adject_tiles(&maze.tile_map)
                .iter()
                .filter(|tile| start_tile.can_connect(tile))
            {
                let mut current_tile = connecting_adjacent_tile.to_owned();
                let mut previous_tile = *start_tile;
                let mut steps: i64 = 0;

                loop {
                    let adjacent_tiles = current_tile.get_adject_tiles(&maze.tile_map);
                    let next_tile: Vec<Tile> = adjacent_tiles
                        .iter()
                        .filter(|&&tile| tile != previous_tile && current_tile.is_connected(&tile))
                        .cloned()
                        .collect();
                    println!("{next_tile:?}");

                    if next_tile.is_empty() {
                        break;
                    }

                    previous_tile = current_tile;
                    current_tile = next_tile[0];
                    steps += 1;
                }
                println!("{steps}");
            }
            12
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
                                TileType::SouthToWestBend
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A model to represent the tiles in the maze
struct Tile {
    /// (x, y) coordinates of the tile in the maze(2-D)
    coordinates: (i64, i64),
    /// The type of tile it is
    value: TileType,
}
impl Tile {
    fn is_connected(self, other: &Tile) -> bool {
        let delta = (
            (self.coordinates.0 - other.coordinates.0),
            (self.coordinates.1 - other.coordinates.1),
        );
        println!("delta: {delta:?} | self: {self:?} | other: {other:?}");

        match (delta, self.value, other.value) {
            ((-1, 0), TileType::VerticalPipe, TileType::VerticalPipe) => true,
            ((-1, 0), TileType::VerticalPipe, TileType::NorthToEastBend) => true,
            ((-1, 0), TileType::VerticalPipe, TileType::NorthToWestBend) => true,
            ((0, -1), TileType::NorthToEastBend, TileType::SouthToWestBend) => true,
            ((0, -1), TileType::NorthToEastBend, TileType::HorizontalPipe) => true,
            ((0, -1), TileType::NorthToEastBend, TileType::NorthToWestBend) => true,
            ((-1, 0), TileType::SouthToWestBend, TileType::NorthToWestBend) => true,
            ((-1, 0), TileType::SouthToWestBend, TileType::VerticalPipe) => true,
            ((-1, 0), TileType::SouthToWestBend, TileType::NorthToWestBend) => true,
            ((0, 1), TileType::NorthToWestBend, TileType::SouthToEastBend) => true,
            ((0, 1), TileType::NorthToWestBend, TileType::HorizontalPipe) => true,
            ((0, 1), TileType::NorthToWestBend, TileType::NorthToEastBend) => true,
            ((-1, 0), TileType::SouthToEastBend, TileType::NorthToWestBend) => true,
            ((-1, 0), TileType::SouthToEastBend, TileType::VerticalPipe) => true,
            ((-1, 0), TileType::SouthToEastBend, TileType::NorthToEastBend) => true,
            ((1, 0), TileType::NorthToWestBend, TileType::VerticalPipe) => true,
            ((1, 0), TileType::NorthToWestBend, TileType::SouthToEastBend) => true,
            ((1, 0), TileType::NorthToWestBend, TileType::SouthToWestBend) => true,
            ((1, 0), TileType::VerticalPipe, TileType::VerticalPipe) => true,
            ((1, 0), TileType::VerticalPipe, TileType::SouthToEastBend) => true,
            ((1, 0), TileType::VerticalPipe, TileType::SouthToWestBend) => true,

            _ => false,
        }
    }

    /// Check if a tile can connect to another tile
    fn can_connect(self, other: &Tile) -> bool {
        let possible_connection_types = match (
            (self.coordinates.0 - other.coordinates.0),
            (self.coordinates.1 - other.coordinates.1),
        ) {
            (-1, 0) => vec![
                TileType::NorthToEastBend,
                TileType::NorthToWestBend,
                TileType::VerticalPipe,
            ],
            (1, 0) => vec![
                TileType::SouthToWestBend,
                TileType::SouthToEastBend,
                TileType::VerticalPipe,
            ],
            (0, -1) => vec![
                TileType::NorthToWestBend,
                TileType::SouthToWestBend,
                TileType::HorizontalPipe,
            ],
            (0, 1) => vec![
                TileType::NorthToEastBend,
                TileType::SouthToEastBend,
                TileType::HorizontalPipe,
            ],
            _ => vec![], // otherwise set it empty so it returns false
        };

        possible_connection_types.contains(&other.value)
    }

    /// Get the tiles [above, below, left, and right] of a tile
    fn get_adject_tiles(self, tile_map: &Vec<Vec<Tile>>) -> Vec<Tile> {
        [
            (self.coordinates.0 - 1, self.coordinates.1),
            (self.coordinates.0 + 1, self.coordinates.1),
            (self.coordinates.0, self.coordinates.1 - 1),
            (self.coordinates.0, self.coordinates.1 + 1),
        ]
        .iter()
        .map(|(x, y)| tile_map[*x as usize][*y as usize])
        .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
