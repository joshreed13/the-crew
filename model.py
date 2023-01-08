from dataclasses import dataclass, field
from enum import Enum

PlayerIndex = int


class Suit(Enum):
    Blue = 0
    Yellow = 1
    Magenta = 2
    Green = 3
    Rocket = 4


@dataclass
class Card:
    suit: Suit
    num: int

    def __repr__(self):
        return f"{{{self.suit.name} {self.num}}}"


@dataclass
class Task:
    player: PlayerIndex
    card: Card


@dataclass
class Objective:
    complete: bool = False


@dataclass
class TaskObjective(Objective):
    absoluteTasks: list[Task] = field(default_factory=list)
    relativeTasks: list[Task] = field(default_factory=list)
    anytimeTasks: list[Task] = field(default_factory=list)
    lastTask: Task | None = None


@dataclass
class PlayerState:
    name: str
    hand: list[Card] = field(default_factory=list)
    played: Card | None = None
    collected: list[Card] = field(default_factory=list)


@dataclass
class GameState:
    players: list[PlayerState] = field(default_factory=list)
    objectives: list[Objective] = field(default_factory=list)
    currentLeader: PlayerIndex = 0
