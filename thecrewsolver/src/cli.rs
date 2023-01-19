use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, CardSet},
    play::Hands,
    player::PlayerIndex,
    solver::GameState,
    tasks::{Task, TasksObjective},
};

#[derive(Serialize, Deserialize)]
struct TaskDTO {
    task_type: String,
    order: u8,
    card: String,
    player_num: u8,
}

#[derive(Serialize, Deserialize)]
struct State {
    hands: Vec<Vec<String>>,
    tasks: Vec<TaskDTO>,
    curr_leader: PlayerIndex,
}

#[derive(Serialize, Deserialize)]
pub struct RunOutput {
    success: bool,
    result: bool,
    duration: u128,
}

impl RunOutput {
    pub fn new(success: bool, result: bool, duration: u128) -> Self {
        Self {
            success,
            result,
            duration,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn parse(input: &str) -> Option<GameState> {
    let state: State = serde_json::from_str(input).ok()?;

    let hands: Hands = state
        .hands
        .iter()
        .map(|h| to_card_set(&h.iter().map(|c| c.as_ref()).collect::<Vec<_>>()))
        .collect::<Option<Vec<_>>>()?
        .try_into()
        .ok()?;

    let absolute_tasks = tasks_of_type(&state.tasks, "absolute")?;
    let relative_tasks = tasks_of_type(&state.tasks, "relative")?;
    let anytime_tasks = tasks_of_type(&state.tasks, "anytime")?;
    let last_task = tasks_of_type(&state.tasks, "last")?.first().copied();
    let tasks = TasksObjective::new(&absolute_tasks, &relative_tasks, &anytime_tasks, last_task);

    Some(GameState::new(hands, tasks, state.curr_leader))
}

fn to_card_set(cards: &[&str]) -> Option<CardSet> {
    let cards: Vec<Card> = cards
        .iter()
        .map(|c| str_to_card(c))
        .collect::<Option<Vec<_>>>()?;
    Some(CardSet::from_cards(&cards))
}

fn str_to_card(card_str: &str) -> Option<Card> {
    let mut iter = card_str.chars();
    let suit = iter.next()?;
    let value = iter.next()?.to_string().parse::<u8>().ok()? as usize;
    iter.next().is_none().then_some(0)?;

    (value > 0).then_some(0)?;

    use crate::card::Card::*;
    match suit {
        'B' => Some(*[B1, B2, B3, B4, B5, B6, B7, B8, B9].get(value - 1)?),
        'Y' => Some(*[Y1, Y2, Y3, Y4, Y5, Y6, Y7, Y8, Y9].get(value - 1)?),
        'M' => Some(*[M1, M2, M3, M4, M5, M6, M7, M8, M9].get(value - 1)?),
        'G' => Some(*[G1, G2, G3, G4, G5, G6, G7, G8, G9].get(value - 1)?),
        'R' => Some(*[R1, R2, R3, R4].get(value - 1)?),
        _ => None,
    }
}

fn tasks_of_type(tasks: &[TaskDTO], task_type: &str) -> Option<Vec<Task>> {
    let mut vec = tasks
        .iter()
        .filter(|t| t.task_type == task_type)
        .collect::<Vec<_>>();

    vec.sort_by(|a, b| a.order.cmp(&b.order));

    vec.iter()
        .map(|t| Some(Task::new(t.player_num, str_to_card(&t.card)?)))
        .collect::<Option<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::Card::*;
    use super::*;

    #[test]
    fn test_parse() {
        let json = r#"{"hands":[["B3"], ["B9"], ["G3"], ["G2"]], "tasks":[{"task_type":"absolute","order":0,"card":"G2","player_num":1}],"curr_leader":0}"#;

        assert_eq!(
            parse(json),
            Some(GameState::new(
                [
                    CardSet::from_cards(&[B3]),
                    CardSet::from_cards(&[B9]),
                    CardSet::from_cards(&[G3]),
                    CardSet::from_cards(&[G2])
                ],
                TasksObjective::new(&[Task::new(1, G2)], &[], &[], None),
                0
            ))
        );
    }

    #[test]
    fn test_str_to_card() {
        assert_eq!(str_to_card("B1"), Some(B1));
        assert_eq!(str_to_card("B9"), Some(B9));
        assert_eq!(str_to_card("Y1"), Some(Y1));
        assert_eq!(str_to_card("Y9"), Some(Y9));
        assert_eq!(str_to_card("M1"), Some(M1));
        assert_eq!(str_to_card("M9"), Some(M9));
        assert_eq!(str_to_card("G1"), Some(G1));
        assert_eq!(str_to_card("G9"), Some(G9));
        assert_eq!(str_to_card("R1"), Some(R1));
        assert_eq!(str_to_card("R4"), Some(R4));
        assert_eq!(str_to_card(""), None);
        assert_eq!(str_to_card("B"), None);
        assert_eq!(str_to_card("R"), None);
        assert_eq!(str_to_card("1"), None);
        assert_eq!(str_to_card("11"), None);
        assert_eq!(str_to_card("R5"), None);
        assert_eq!(str_to_card("R9"), None);
        assert_eq!(str_to_card("1B"), None);
        assert_eq!(str_to_card("B0"), None);
        assert_eq!(str_to_card("Y0"), None);
        assert_eq!(str_to_card("M0"), None);
        assert_eq!(str_to_card("G0"), None);
        assert_eq!(str_to_card("R0"), None);
        assert_eq!(str_to_card("B11"), None);
        assert_eq!(str_to_card("asdf"), None);
    }

    #[test]
    fn test_to_card_set() {
        assert_eq!(
            to_card_set(&["B6", "M9"]),
            Some(CardSet::from_cards(&[B6, M9]))
        );
        assert_eq!(to_card_set(&["B0", "M9"]), None);
    }

    #[test]
    fn test_tasks_of_type() {
        fn t(task_type: &str, order: u8, card: &str, player_num: u8) -> TaskDTO {
            TaskDTO {
                task_type: task_type.to_string(),
                order,
                card: card.to_string(),
                player_num,
            }
        }

        assert_eq!(tasks_of_type(&[], "absolute"), Some(vec![]));
        assert_eq!(
            tasks_of_type(
                &[
                    t("absolute", 1, "B1", 1),
                    t("relative", 0, "Y2", 0),
                    t("absolute", 2, "M3", 3),
                ],
                "absolute"
            ),
            Some(vec![Task::new(1, B1), Task::new(3, M3)])
        );
        assert_eq!(
            tasks_of_type(
                &[t("absolute", 2, "M3", 3), t("absolute", 1, "B1", 1)],
                "absolute"
            ),
            Some(vec![Task::new(1, B1), Task::new(3, M3)])
        );
    }
}
