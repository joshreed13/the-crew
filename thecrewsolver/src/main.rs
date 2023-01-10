mod card;
mod play;
mod player;
mod solver;
mod tasks;

use crate::card::{card_list_to_set, CardSet};
use crate::player::NUM_PLAYERS;
use crate::solver::{solve, GameState};
use crate::tasks::{Task, TaskListBuilder, TasksObjective};

fn main() {
    use card::cards::*;
    let hands: [CardSet; NUM_PLAYERS] = [
        card_list_to_set(&[B3, G1, M6, R4, B4, M3, Y2, Y6, B5, G9]),
        card_list_to_set(&[Y4, M5, G8, M4, G7, B1, R3, M7, Y7, B9]),
        card_list_to_set(&[Y9, G5, Y5, M1, M9, Y3, B7, M2, M8, G3]),
        card_list_to_set(&[R2, Y8, Y1, B6, B2, R1, G6, B8, G4, G2]),
    ];
    let tasks = TasksObjective::new(
        TaskListBuilder::from_list(&[Task::new(0, B8)]).done(),
        TaskListBuilder::from_list(&[]).done(),
        TaskListBuilder::from_list(&[Task::new(1, G7), Task::new(2, M5), Task::new(3, Y4)]).done(),
        None,
    );

    println!("Calculating...");
    let result = solve(&GameState::new(hands, tasks, 0));
    println!("{}", result);
}
