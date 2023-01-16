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
class Player:
    name: str
    isCommander: bool


@dataclass_json
@dataclass
class Turn:
    player: Player
    card: Optional[Card]
    isLeader: bool
    isWinner: bool
    isNextToPlay: bool


@dataclass_json
@dataclass
class Trick:
    turns: list[Turn]


@dataclass_json
@dataclass
class Task:
    id: str
    type: str
    order: int
    card: Optional[Card]
    player: Optional[Player]


@dataclass_json
@dataclass
class PlayerState:
    player: Player
    hand: list[Card]
    tasks: list[Task]


@dataclass_json
@dataclass
class HandPageState:
    heldCards: list[Card]


@dataclass_json
@dataclass
class ObjectivePageState:
    tasks: list[Task]


@dataclass_json
@dataclass
class TricksPageState:
    tricks: list[Trick]


@dataclass_json
@dataclass
class ControlPanelState:
    players: list[PlayerState]
    tricks: list[Trick]


@dataclass_json
@dataclass
class AppState:
    handPage: HandPageState
    objectivePage: ObjectivePageState
    tricksPage: TricksPageState
    controlPanel: ControlPanelState
