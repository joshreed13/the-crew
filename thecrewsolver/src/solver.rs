use crate::card::CardSet;
use crate::play::{Hands, Play, PlayGenerator};
use crate::player::PlayerIndex;
use crate::tasks::TasksObjective;

#[derive(PartialEq, Debug)]
pub struct GameState {
    hands: Hands,
    tasks: TasksObjective,
    curr_leader: PlayerIndex,
}

impl GameState {
    pub fn new(hands: Hands, tasks: TasksObjective, curr_leader: PlayerIndex) -> GameState {
        GameState {
            hands,
            tasks,
            curr_leader,
        }
    }
}

pub fn solve(state: &GameState) -> bool {
    let generator = PlayGenerator::new(&state.hands, state.curr_leader);
    for play in generator {
        let result = solve_play(state, &play);
        if result {
            return result;
        }
    }
    false
}

fn solve_play(state: &GameState, play: &Play) -> bool {
    let winning_card = play.get_trick_winner();
    let winner = find_player_with_card(&state.hands, winning_card).unwrap();

    let new_tasks = state.tasks.check(play.cards(), winner);

    match new_tasks {
        None => false,
        Some(new_tasks) => {
            if new_tasks.is_complete() {
                true
            } else {
                let remaining_hands = get_remaining_hands(&state.hands, play);
                let new_state = GameState::new(remaining_hands, new_tasks, winner);
                solve(&new_state)
            }
        }
    }
}

fn find_player_with_card(hands: &Hands, card: CardSet) -> Option<PlayerIndex> {
    for (i, hand) in hands.iter().enumerate() {
        if hand.overlaps_with(card) {
            return Some(i as PlayerIndex);
        }
    }
    None
}

fn get_remaining_hands(hands: &Hands, play: &Play) -> Hands {
    hands.map(|x| x & !play.cards())
}
