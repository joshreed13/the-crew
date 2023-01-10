mod cards;

fn main() {
    use cards::*;
    let hands: [CardSet; NUM_PLAYERS] = [
        card_index_to_set(&[B9]),
        card_index_to_set(&[Y4]),
        card_index_to_set(&[Y9]),
        card_index_to_set(&[B8]),
        /*card_index_to_set(&[B3, G1, M6, R4, B4, M3, Y2, Y6, B5, G9]),
        card_index_to_set(&[Y4, M5, G8, M4, G7, B1, R3, M7, Y7, B9]),
        card_index_to_set(&[Y9, G5, Y5, M1, M9, Y3, B7, M2, M8, G3]),
        card_index_to_set(&[R2, Y8, Y1, B6, B2, R1, G6, B8, G4, G2]),*/
    ];
    let tasks = TasksObjective {
        absolute_tasks: TaskListBuilder::from_list(&[Task::new(0, B8)]).done(),
        relative_tasks: TaskListBuilder::from_list(&[]).done(),
        anytime_tasks: TaskListBuilder::from_list(&[
            Task::new(1, G7),
            Task::new(2, M5),
            Task::new(3, Y4),
        ])
        .done(),
        last_task: None,
    };

    println!("Calculating...");
    let result = solve_step(&GameState::new(hands, tasks, 0));
    println!("{}", result);
}

const NUM_PLAYERS: usize = 4;

type CardSet = u64;
type PlayerIndex = u8;
type CardIndex = u8;

mod suit {
    use crate::CardSet;

    pub const BLUE: CardSet = 0b111111111;
    pub const YELLOW: CardSet = 0b111111111 << 9;
    pub const MAGENTA: CardSet = 0b111111111 << 18;
    pub const GREEN: CardSet = 0b111111111 << 27;
    pub const ROCKETS: CardSet = 0b1111 << 36;
}

pub struct GameState {
    hands: [CardSet; NUM_PLAYERS],
    tasks: TasksObjective,
    curr_leader: PlayerIndex,
}

impl GameState {
    fn new(
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

pub struct TasksObjective {
    absolute_tasks: TaskList,
    relative_tasks: TaskList,
    anytime_tasks: TaskList,
    last_task: Option<Task>,
}

pub struct TaskList {
    mask: CardSet,
    tasks: [Task; 12],
}

#[derive(Clone, Copy, Default)]
pub struct Task {
    player: PlayerIndex,
    card: CardIndex,
}

impl TasksObjective {
    fn check(&self, play: CardSet, winner: PlayerIndex) -> Option<TasksObjective> {
        if self.check_order(play) {
            Some(TasksObjective {
                absolute_tasks: self.absolute_tasks.check_completed_front(play, winner)?,
                relative_tasks: self.relative_tasks.check_completed_front(play, winner)?,
                anytime_tasks: self.anytime_tasks.check_completed_any(play, winner)?,
                last_task: self.check_completed_last(play, winner)?,
            })
        } else {
            None
        }
    }

    fn check_order(&self, play: CardSet) -> bool {
        let abs_done = self.absolute_tasks.covered_by(play);
        let rel_done = self.relative_tasks.covered_by(play);
        let any_done = self.anytime_tasks.covered_by(play);
        let last_done = self.last_is_covered_by(play);

        let rel_is_good = !rel_done || (abs_done);
        let any_is_good = !any_done || (abs_done);
        let last_is_good = !last_done || (abs_done && rel_done && any_done);

        rel_is_good && any_is_good && last_is_good
    }

    fn last_is_covered_by(&self, cards: CardSet) -> bool {
        match &self.last_task {
            Some(task) => card_at_position(task.card) & !cards == 0,
            None => true,
        }
    }

    fn check_completed_last(&self, play: CardSet, winner: PlayerIndex) -> Option<Option<Task>> {
        match &self.last_task {
            Some(task) => match task.evaluate(play, winner) {
                TaskEvaluation::Complete => Some(None),
                TaskEvaluation::Failed => None,
                TaskEvaluation::InProgress => Some(self.last_task),
            },
            None => Some(None),
        }
    }

    fn is_complete(&self) -> bool {
        self.absolute_tasks.is_complete()
            && self.relative_tasks.is_complete()
            && self.anytime_tasks.is_complete()
            && self.last_task.is_none()
    }
}

struct TaskListBuilder {
    i: usize,
    list: TaskList,
}

impl TaskListBuilder {
    fn new() -> TaskListBuilder {
        TaskListBuilder {
            i: 0,
            list: TaskList {
                mask: 0,
                tasks: Default::default(),
            },
        }
    }

    fn from_list(tasks: &[Task]) -> TaskListBuilder {
        let mut builder = TaskListBuilder::new();
        for t in tasks {
            builder.push(*t);
        }
        builder
    }

    fn push(&mut self, task: Task) {
        self.list.mask |= card_at_position(task.card);
        self.list.tasks[self.i] = task;
        self.i += 1;
    }

    fn done(self) -> TaskList {
        self.list
    }
}

impl TaskList {
    fn is_complete(&self) -> bool {
        self.mask == 0
    }

    fn covered_by(&self, cards: CardSet) -> bool {
        self.mask & !cards == 0
    }

    fn check_completed_front(&self, play: CardSet, winner: PlayerIndex) -> Option<TaskList> {
        let mut builder = TaskListBuilder::new();
        let mut no_more = false;
        for task in self.tasks {
            if !no_more {
                match task.evaluate(play, winner) {
                    TaskEvaluation::Complete => {}
                    TaskEvaluation::Failed => {
                        return None;
                    }
                    TaskEvaluation::InProgress => {
                        builder.push(task);
                        no_more = true;
                    }
                }
            } else {
                match task.evaluate(play, winner) {
                    TaskEvaluation::Complete => {
                        return None;
                    }
                    TaskEvaluation::Failed => {
                        return None;
                    }
                    TaskEvaluation::InProgress => {
                        builder.push(task);
                    }
                }
            }
        }
        Some(builder.done())
    }

    fn check_completed_any(&self, play: CardSet, winner: PlayerIndex) -> Option<TaskList> {
        let mut builder = TaskListBuilder::new();
        for task in &self.tasks {
            match task.evaluate(play, winner) {
                TaskEvaluation::Complete => {}
                TaskEvaluation::Failed => {
                    return None;
                }
                TaskEvaluation::InProgress => {
                    builder.push(*task);
                }
            }
        }

        Some(builder.done())
    }
}

enum TaskEvaluation {
    Failed,
    InProgress,
    Complete,
}

impl Task {
    fn new(player: PlayerIndex, card: CardIndex) -> Task {
        Task { player, card }
    }

    fn matches(&self, play: CardSet) -> bool {
        let card: CardSet = 1 << self.card;
        play & card != 0
    }

    fn evaluate(&self, play: CardSet, winner: PlayerIndex) -> TaskEvaluation {
        if self.matches(play) {
            if self.player == winner {
                TaskEvaluation::Complete
            } else {
                TaskEvaluation::Failed
            }
        } else {
            TaskEvaluation::InProgress
        }
    }
}

fn get_trick_winner(play: &Play) -> CardSet {
    let eligible_cards = play.lead_suit | suit::ROCKETS;
    let relevant_cards = play.cards & eligible_cards;
    get_high_card(relevant_cards)
}

fn get_suit(card: CardSet) -> CardSet {
    if card & suit::BLUE != 0 {
        suit::BLUE
    } else if card & suit::YELLOW != 0 {
        suit::YELLOW
    } else if card & suit::MAGENTA != 0 {
        suit::MAGENTA
    } else if card & suit::GREEN != 0 {
        suit::GREEN
    } else if card & suit::ROCKETS != 0 {
        suit::ROCKETS
    } else {
        0
    }
}

fn get_high_card(cards: CardSet) -> CardSet {
    1 << (CardSet::BITS - cards.leading_zeros())
}

struct PositionCounter {
    hand: CardSet,
    mask: CardSet,
    position: CardIndex,
}

impl PositionCounter {
    fn new(hand: CardSet) -> PositionCounter {
        let initial_pos: CardIndex = hand.trailing_zeros() as CardIndex;
        PositionCounter {
            hand: hand.rotate_right(initial_pos as u32),
            mask: 0,
            position: initial_pos,
        }
    }

    fn increment(&mut self) -> bool {
        self.hand = self.hand.rotate_right(1);
        let next_step = self.hand.trailing_zeros() as u8;
        self.hand = self.hand.rotate_right(next_step as u32);
        self.position += next_step + 1;
        let overflowed: bool = self.position > CardSet::BITS as u8;
        self.position %= CardSet::BITS as u8;
        overflowed
    }

    fn reset_with_mask(&mut self, mask: CardSet) {
        self.mask = mask;
        self.hand = self.hand.rotate_left(self.position as u32);
        self.position = 0;
        self.increment();
    }
}

struct Play {
    cards: CardSet,
    lead_suit: CardSet,
}

struct PlayGenerator {
    counters: [PositionCounter; NUM_PLAYERS],
    lead_suit: CardSet,
}

impl PlayGenerator {
    fn new(hands: &[CardSet; NUM_PLAYERS], lead_player: PlayerIndex) -> PlayGenerator {
        let mut obj = PlayGenerator {
            counters: hands.map(PositionCounter::new),
            lead_suit: 0,
        };
        obj.counters.rotate_left(lead_player as usize);
        obj.set_lead_suit();
        obj
    }

    fn build_cards(&self) -> CardSet {
        self.counters
            .iter()
            .fold(0, |a, x| a | card_at_position(x.position))
    }

    fn set_lead_suit(&mut self) {
        self.lead_suit = get_suit(card_at_position(self.counters[0].position));
        for counter in &mut self.counters[1..] {
            let must_follow_suit = counter.hand & self.lead_suit != 0;
            let mask = if must_follow_suit {
                self.lead_suit
            } else {
                CardSet::MAX
            };
            counter.reset_with_mask(mask);
        }
    }

    fn get_play(&self) -> Play {
        Play {
            cards: self.build_cards(),
            lead_suit: self.lead_suit,
        }
    }
}

impl Iterator for PlayGenerator {
    type Item = Play;

    fn next(&mut self) -> Option<Play> {
        for counter in &mut self.counters[1..].iter_mut().rev() {
            let overflowed = counter.increment();
            if !overflowed {
                return Some(self.get_play());
            }
        }

        let overflowed = self.counters[0].increment();
        if overflowed {
            None
        } else {
            self.set_lead_suit();
            Some(self.get_play())
        }
    }
}

fn card_at_position(index: CardIndex) -> CardSet {
    1 << index
}

fn card_index_to_set(cards: &[CardIndex]) -> CardSet {
    cards.iter().fold(0, |a, c| a | card_at_position(*c))
}
fn solve_step(state: &GameState) -> bool {
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
                solve_step(&new_state)
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
