use std::fs;

mod moves;

pub use moves::{move_map, Move, MoveKind};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("uh oh");
    let rounds: Vec<&str> = input.split("\n").filter(|&x| !x.is_empty()).collect();
    let mut _sum: i32 = 0;

    for round in rounds {
        let moves = move_map();
        let scenario: Vec<&str> = round.split(" ").collect();

        let enemy_move = moves[scenario[0]];
        let my_move = moves[scenario[1]];

        if enemy_move.beats == my_move.kind {
            _sum += my_move.points;
        } else if my_move.beats == enemy_move.kind {
            _sum += my_move.points + 6;
        } else {
            _sum += my_move.points + 3;
        }
    }

    println!("Answer for part 1 is: {}", _sum);
}
