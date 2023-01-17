from dataclasses import dataclass
from dataclasses_json import dataclass_json
from typing import Optional


@dataclass_json
@dataclass
class Card:
    suit: str
    value: int


@dataclass_json
@dataclass
class PlayerState:
    name: str
    hand: list[Card]


@dataclass_json
@dataclass
class Task:
    type: str
    order: int
    card: Optional[Card]
    playerNum: Optional[int]


@dataclass_json
@dataclass
class Trick:
    turns: list[Optional[Card]]


@dataclass_json
@dataclass
class RoundState:
    players: list[PlayerState]
    objectives: dict[int, Task]
    tricks: list[Trick]
