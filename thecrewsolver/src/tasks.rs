use crate::card::{Card, CardSet};
use crate::player::PlayerIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Task {
    player: PlayerIndex,
    card: Card,
}

impl Task {
    pub fn new(player: PlayerIndex, card: Card) -> Task {
        Task { player, card }
    }

    fn matches(&self, play: CardSet) -> bool {
        play.contains(self.card)
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

pub struct TasksObjective {
    absolute_tasks: TaskList,
    relative_tasks: TaskList,
    anytime_tasks: TaskList,
    last_task: Option<Task>,
}

impl TasksObjective {
    pub fn new(
        absolute_tasks: &[Task],
        relative_tasks: &[Task],
        anytime_tasks: &[Task],
        last_task: Option<Task>,
    ) -> Self {
        Self {
            absolute_tasks: TaskListBuilder::from_list(absolute_tasks).done(),
            relative_tasks: TaskListBuilder::from_list(relative_tasks).done(),
            anytime_tasks: TaskListBuilder::from_list(anytime_tasks).done(),
            last_task,
        }
    }

    pub fn check(&self, play: CardSet, winner: PlayerIndex) -> Option<TasksObjective> {
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
        let have_rel = !self.relative_tasks.is_complete();
        let have_any = !self.anytime_tasks.is_complete();
        let have_last = self.last_task.is_some();

        let abs_done = self.absolute_tasks.covered_by(play);
        let rel_done = self.relative_tasks.covered_by(play);
        let any_done = self.anytime_tasks.covered_by(play);
        let last_done = self.last_is_covered_by(play);

        let rel_is_good = !have_rel || (!rel_done || abs_done);
        let any_is_good = !have_any || (!any_done || abs_done);
        let last_is_good = !have_last || (!last_done || (abs_done && rel_done && any_done));

        rel_is_good && any_is_good && last_is_good
    }

    fn last_is_covered_by(&self, cards: CardSet) -> bool {
        match &self.last_task {
            Some(task) => cards.contains(task.card),
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

    pub fn is_complete(&self) -> bool {
        self.absolute_tasks.is_complete()
            && self.relative_tasks.is_complete()
            && self.anytime_tasks.is_complete()
            && self.last_task.is_none()
    }
}

enum TaskEvaluation {
    Failed,
    InProgress,
    Complete,
}

const TASK_LIST_LENGTH: usize = 12;

pub struct TaskList {
    mask: CardSet,
    tasks: [Task; TASK_LIST_LENGTH],
}

impl TaskList {
    fn is_complete(&self) -> bool {
        self.mask == CardSet::EMPTY
    }

    fn covered_by(&self, cards: CardSet) -> bool {
        self.mask.is_covered_by(cards)
    }

    fn valid_tasks(&self) -> &[Task] {
        let num_tasks = self.mask.num_set() as usize;
        &self.tasks[..num_tasks]
    }

    fn check_completed_front(&self, play: CardSet, winner: PlayerIndex) -> Option<TaskList> {
        let mut builder = TaskListBuilder::new();
        let mut no_more = false;
        for task in self.valid_tasks() {
            if !no_more {
                match task.evaluate(play, winner) {
                    TaskEvaluation::Complete => {}
                    TaskEvaluation::Failed => {
                        return None;
                    }
                    TaskEvaluation::InProgress => {
                        builder.push(*task);
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
                        builder.push(*task);
                    }
                }
            }
        }
        Some(builder.done())
    }

    fn check_completed_any(&self, play: CardSet, winner: PlayerIndex) -> Option<TaskList> {
        let mut builder = TaskListBuilder::new();
        for task in self.valid_tasks() {
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

pub struct TaskListBuilder {
    i: usize,
    list: TaskList,
}

impl TaskListBuilder {
    pub fn new() -> TaskListBuilder {
        TaskListBuilder {
            i: 0,
            list: TaskList {
                mask: CardSet::EMPTY,
                tasks: [Task::new(0, Card::B1); TASK_LIST_LENGTH],
            },
        }
    }

    pub fn from_list(tasks: &[Task]) -> TaskListBuilder {
        let mut builder = TaskListBuilder::new();
        for t in tasks {
            builder.push(*t);
        }
        builder
    }

    pub fn push(&mut self, task: Task) {
        self.list.mask = self.list.mask.add(CardSet::from_card(task.card));
        self.list.tasks[self.i] = task;
        self.i += 1;
    }

    pub fn done(self) -> TaskList {
        self.list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card::*;

    #[test]
    fn test_task_list_builder() {
        let mut tlb = TaskListBuilder::new();
        tlb.push(Task::new(2, Y5));
        tlb.push(Task::new(4, M2));
        tlb.push(Task::new(0, B1));

        let expected = [Task::new(2, Y5), Task::new(4, M2), Task::new(0, B1)];
        let actual = tlb.done();
        assert_eq!(actual.tasks[0..3], expected);
        assert_eq!(actual.mask, CardSet::from_cards(&[Y5, M2, B1]));
    }

    #[test]
    fn test_task_list_builder_from_list() {
        let tasks = [Task::new(2, G6), Task::new(4, M1)];
        let mut tlb = TaskListBuilder::from_list(&tasks);
        tlb.push(Task::new(0, B9));

        let expected = [Task::new(2, G6), Task::new(4, M1), Task::new(0, B9)];
        let actual = tlb.done();
        assert_eq!(actual.tasks[0..3], expected);
        assert_eq!(actual.mask, CardSet::from_cards(&[G6, M1, B9]));
    }

    #[test]
    fn test_task_list_init() {
        assert!(TaskListBuilder::new().done().is_complete());
    }

    #[test]
    fn test_empty_objective() {
        assert!(TasksObjective::new(&[], &[], &[], None).is_complete());
    }

    #[test]
    fn test_different_players_win() {
        assert!(TasksObjective::new(&[Task::new(0, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[Task::new(1, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 1)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[Task::new(2, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 2)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[Task::new(3, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 3)
            .unwrap()
            .is_complete());
    }

    #[test]
    fn test_wrong_player_doesnt_win() {
        assert!(TasksObjective::new(&[Task::new(0, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 1)
            .is_none());
        assert!(TasksObjective::new(&[Task::new(1, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
            .is_none());
    }

    #[test]
    fn test_absolute_doesnt_have_to_be_immediate() {
        let obj = TasksObjective::new(&[Task::new(0, B8)], &[], &[], None);
        assert!(!obj.is_complete());

        let obj = obj
            .check(CardSet::from_cards(&[B1, B5, B7, M3]), 0)
            .unwrap();
        assert!(!obj.is_complete());

        let obj = obj
            .check(CardSet::from_cards(&[R3, Y8, G2, B8]), 0)
            .unwrap();
        assert!(obj.is_complete());
    }

    #[test]
    fn test_single_absolute_task() {
        assert!(TasksObjective::new(&[Task::new(0, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[Task::new(0, B8)], &[], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 1)
            .is_none());
    }

    #[test]
    fn test_single_relative_task() {
        assert!(TasksObjective::new(&[], &[Task::new(0, B8)], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[], &[Task::new(0, B8)], &[], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 1)
            .is_none());
    }

    #[test]
    fn test_single_anytime_task() {
        assert!(TasksObjective::new(&[], &[], &[Task::new(0, B8)], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[], &[], &[Task::new(0, B8)], None)
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 1)
            .is_none());
    }

    #[test]
    fn test_single_last_task() {
        assert!(TasksObjective::new(&[], &[], &[], Some(Task::new(0, B8)))
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
            .unwrap()
            .is_complete());
        assert!(TasksObjective::new(&[], &[], &[], Some(Task::new(0, B8)))
            .check(CardSet::from_cards(&[B1, B5, B8, M3]), 1)
            .is_none());
    }

    #[test]
    fn test_two_absolute_tasks() {
        assert!(
            TasksObjective::new(&[Task::new(0, B8), Task::new(1, G1)], &[], &[], None)
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[Task::new(0, B8), Task::new(1, G1)], &[], &[], None)
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_two_relative_tasks() {
        assert!(
            TasksObjective::new(&[], &[Task::new(0, B8), Task::new(1, G1)], &[], None)
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[], &[Task::new(0, B8), Task::new(1, G1)], &[], None)
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_two_anytime_tasks() {
        assert!(
            TasksObjective::new(&[], &[], &[Task::new(0, B8), Task::new(1, G1)], None)
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[], &[], &[Task::new(0, B8), Task::new(1, G1)], None)
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_absolute_before_relative() {
        assert!(
            TasksObjective::new(&[Task::new(0, B8)], &[Task::new(1, G1)], &[], None)
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[Task::new(0, B8)], &[Task::new(1, G1)], &[], None)
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_absolute_before_anytime() {
        assert!(
            TasksObjective::new(&[Task::new(0, B8)], &[], &[Task::new(1, G1)], None)
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[Task::new(0, B8)], &[], &[Task::new(1, G1)], None)
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_absolute_before_last() {
        assert!(
            TasksObjective::new(&[Task::new(0, B8)], &[], &[], Some(Task::new(1, G1)))
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[Task::new(0, B8)], &[], &[], Some(Task::new(1, G1)))
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_relative_can_mix_with_anytime() {
        assert!(
            TasksObjective::new(&[], &[Task::new(0, B8)], &[Task::new(1, G1)], None)
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[], &[Task::new(0, B8)], &[Task::new(1, G1)], None)
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_relative_before_last() {
        assert!(
            TasksObjective::new(&[], &[Task::new(0, B8)], &[], Some(Task::new(1, G1)))
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[], &[Task::new(0, B8)], &[], Some(Task::new(1, G1)))
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_anytime_before_last() {
        assert!(
            TasksObjective::new(&[], &[], &[Task::new(0, B8)], Some(Task::new(1, G1)))
                .check(CardSet::from_cards(&[B1, B5, B8, M3]), 0)
                .unwrap()
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .unwrap()
                .is_complete()
        );
        assert!(
            TasksObjective::new(&[], &[], &[Task::new(0, B8)], Some(Task::new(1, G1)))
                .check(CardSet::from_cards(&[M5, G9, Y2, G1]), 1)
                .is_none()
        );
    }

    #[test]
    fn test_two_absolute_one_turn() {
        assert!(
            TasksObjective::new(&[Task::new(0, G1), Task::new(0, B8)], &[], &[], None)
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_two_relative_one_turn() {
        assert!(
            TasksObjective::new(&[], &[Task::new(0, G1), Task::new(0, B8)], &[], None)
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_two_anytime_one_turn() {
        assert!(
            TasksObjective::new(&[], &[], &[Task::new(0, G1), Task::new(0, B8)], None)
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_absolute_and_relative_one_turn() {
        assert!(
            TasksObjective::new(&[Task::new(0, G1)], &[Task::new(0, B8)], &[], None)
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_absolute_and_anytime_one_turn() {
        assert!(
            TasksObjective::new(&[Task::new(0, G1)], &[], &[Task::new(0, B8)], None)
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_absolute_and_last_one_turn() {
        assert!(
            TasksObjective::new(&[Task::new(0, G1)], &[], &[], Some(Task::new(0, B8)))
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_relative_and_anytime_one_turn() {
        assert!(
            TasksObjective::new(&[], &[Task::new(0, G1)], &[Task::new(0, B8)], None)
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_relative_and_last_one_turn() {
        assert!(
            TasksObjective::new(&[], &[Task::new(0, G1)], &[], Some(Task::new(0, B8)))
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_anytime_and_last_one_turn() {
        assert!(
            TasksObjective::new(&[], &[], &[Task::new(0, G1)], Some(Task::new(0, B8)))
                .check(CardSet::from_cards(&[B1, G1, B8, M3]), 0)
                .unwrap()
                .is_complete()
        );
    }

    #[test]
    fn test_all_types_one_turn() {
        assert!(TasksObjective::new(
            &[Task::new(0, G1)],
            &[Task::new(0, M7)],
            &[Task::new(0, Y3)],
            Some(Task::new(0, B8))
        )
        .check(CardSet::from_cards(&[B8, M7, Y3, G1]), 0)
        .unwrap()
        .is_complete());
    }
    #[test]
    fn test_all_types_in_order() {
        assert!(TasksObjective::new(
            &[Task::new(0, G1)],
            &[Task::new(1, M7)],
            &[Task::new(2, Y3)],
            Some(Task::new(3, B8))
        )
        .check(CardSet::from_cards(&[B6, Y8, G2, M1]), 0)
        .unwrap()
        .check(CardSet::from_cards(&[G1, M2, Y1, G7]), 0)
        .unwrap()
        .check(CardSet::from_cards(&[M7, R2, G5, M9]), 1)
        .unwrap()
        .check(CardSet::from_cards(&[B7, Y4, R4, Y3]), 2)
        .unwrap()
        .check(CardSet::from_cards(&[B1, Y2, B8, Y9]), 3)
        .unwrap()
        .is_complete());
    }
}
