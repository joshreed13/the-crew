use crate::card::{card_at_position, CardIndex, CardSet};
use crate::player::PlayerIndex;

#[derive(Clone, Copy, Default)]
pub struct Task {
    player: PlayerIndex,
    card: CardIndex,
}

impl Task {
    pub fn new(player: PlayerIndex, card: CardIndex) -> Task {
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

pub struct TasksObjective {
    absolute_tasks: TaskList,
    relative_tasks: TaskList,
    anytime_tasks: TaskList,
    last_task: Option<Task>,
}

impl TasksObjective {
    pub fn new(
        absolute_tasks: TaskList,
        relative_tasks: TaskList,
        anytime_tasks: TaskList,
        last_task: Option<Task>,
    ) -> Self {
        Self {
            absolute_tasks,
            relative_tasks,
            anytime_tasks,
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

pub struct TaskList {
    mask: CardSet,
    tasks: [Task; 12],
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

pub struct TaskListBuilder {
    i: usize,
    list: TaskList,
}

impl TaskListBuilder {
    pub fn new() -> TaskListBuilder {
        TaskListBuilder {
            i: 0,
            list: TaskList {
                mask: 0,
                tasks: Default::default(),
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
        self.list.mask |= card_at_position(task.card);
        self.list.tasks[self.i] = task;
        self.i += 1;
    }

    pub fn done(self) -> TaskList {
        self.list
    }
}
