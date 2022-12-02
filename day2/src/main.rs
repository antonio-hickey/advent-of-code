use std::collections::HashMap;
use std::fs;

mod moves;

pub use moves::{move_map, move_map_by_kind, Move, MoveKind};

fn solution(enemy_play: &str, my_play: &str, part_number: i8) -> i32 {
    let moves = move_map();
    let enemy_move = moves[enemy_play];

    if part_number == 1 {
        // Part One where I'm assigned my move (Rock, Paper, Scissor)
        let mut sum = 0;
        let my_move = moves[my_play];

        if enemy_move.beats == my_move.kind {
            sum += my_move.points;
        } else if my_move.beats == enemy_move.kind {
            sum += my_move.points + 6;
        } else {
            sum += my_move.points + 3;
        }

        return sum;

    } else {
        //Part Two where I'm assigned my outcome (win, lose, draw)
        let moves_by_kind = move_map_by_kind();

        let scenario_map = HashMap::from([
            ("X", Vec::from([moves_by_kind[&enemy_move.beats].points, 0])),
            ("Y", Vec::from([moves_by_kind[&enemy_move.draws].points, 3])),
            ("Z", Vec::from([moves_by_kind[&enemy_move.losses].points, 6])),
        ]);

        return scenario_map[my_play].iter().sum();
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("uh oh");
    let rounds: Vec<&str> = input.split("\n").filter(|&x| !x.is_empty()).collect();
    let mut _part_one_sum: i32 = 0;
    let mut _part_two_sum: i32 = 0;

    for round in rounds {
        let scenario: Vec<&str> = round.split(" ").collect();
        _part_one_sum += solution(scenario[0], scenario[1], 1);
        _part_two_sum += solution(scenario[0], scenario[1], 2);
    }

    println!("Answer for part 1 is: {}", _part_one_sum);
    println!("Answer for part 2 is: {}", _part_two_sum);
}
