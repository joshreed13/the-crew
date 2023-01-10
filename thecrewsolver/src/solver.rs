use crate::card::{CardIndex, CardSet};
use crate::play::{get_trick_winner, Play, PlayGenerator};
use crate::player::{PlayerIndex, NUM_PLAYERS};
use crate::tasks::TasksObjective;

pub struct GameState {
    hands: [CardSet; NUM_PLAYERS],
    tasks: TasksObjective,
    curr_leader: PlayerIndex,
}

impl GameState {
    pub fn new(
        hands: [CardSet; NUM_PLAYERS],
        tasks: TasksObjective,
        curr_leader: PlayerIndex,
    ) -> GameState {
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
    let winning_card = get_trick_winner(play);
    let winner = find_player_with_card(&state.hands, winning_card).unwrap();

    let new_tasks = state.tasks.check(play.cards, winner);

    match new_tasks {
        None => false,
        Some(new_tasks) => {
            if new_tasks.is_complete() {
                true
            } else {
                let remaining_hands = get_remaining_hands(&state.hands, play.cards);
                let new_state = GameState::new(remaining_hands, new_tasks, winner);
                solve(&new_state)
            }
        }
    }
}

fn find_player_with_card(hands: &[CardSet; NUM_PLAYERS], card: CardSet) -> Option<CardIndex> {
    for (i, hand) in hands.iter().enumerate() {
        if hand & card != 0 {
            return Some(i as CardIndex);
        }
    }
    None
}

fn get_remaining_hands(hands: &[CardSet; NUM_PLAYERS], play: CardSet) -> [CardSet; NUM_PLAYERS] {
    hands.map(|x| x & !play)
}
