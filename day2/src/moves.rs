use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MoveKind {
    Rock,
    Paper,
    Scissor,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Move {
    pub kind: MoveKind,
    pub draws: MoveKind,
    pub beats: MoveKind,
    pub losses: MoveKind,
    pub points: i32,
}

pub const ROCK: Move = Move {
    kind: MoveKind::Rock,
    draws: MoveKind::Rock,
    beats: MoveKind::Scissor,
    losses: MoveKind::Paper,
    points: 1,
};

pub const PAPER: Move = Move {
   kind: MoveKind::Paper,
   draws: MoveKind::Paper,
   beats: MoveKind::Rock,
   losses: MoveKind::Scissor,
   points: 2,
};

pub const SCISSOR: Move = Move {
    kind: MoveKind::Scissor,
    draws: MoveKind::Scissor,
    beats: MoveKind::Paper,
    losses: MoveKind::Rock,
    points: 3,
};


pub fn move_map() -> HashMap<String, Move> {
    HashMap::from([
        (String::from("A"), ROCK),
        (String::from("X"), ROCK),
        (String::from("B"), PAPER),
        (String::from("Y"), PAPER),
        (String::from("C"), SCISSOR),
        (String::from("Z"), SCISSOR),
    ])
}
