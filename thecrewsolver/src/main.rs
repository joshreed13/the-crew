mod card;
mod play;
mod player;
mod solver;
mod tasks;

use crate::card::CardSet;
use crate::play::Hands;
use crate::solver::{solve, GameState};
use crate::tasks::{Task, TasksObjective};

fn main() {
    use card::Card::*;
    let hands: Hands = [
        CardSet::from_cards(&[B3, G1, M6, R4, B4, M3, Y2, Y6, B5, G9]),
        CardSet::from_cards(&[Y4, M5, G8, M4, G7, B1, R3, M7, Y7, B9]),
        CardSet::from_cards(&[Y9, G5, Y5, M1, M9, Y3, B7, M2, M8, G3]),
        CardSet::from_cards(&[R2, Y8, Y1, B6, B2, R1, G6, B8, G4, G2]),
    ];
    let tasks = TasksObjective::new(
        &[Task::new(0, B8)],
        &[],
        &[Task::new(1, G7), Task::new(2, M5), Task::new(3, Y4)],
        None,
    );

    println!("Calculating...");
    let result = solve(&GameState::new(hands, tasks, 0));
    println!("{}", result);
}
