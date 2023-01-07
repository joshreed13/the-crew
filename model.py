from dataclasses import dataclass, field
from enum import Enum


class Suit(Enum):
    Blue = 0
    Yellow = 1
    Magenta = 2
    Green = 3
    Rocket = 4


class Token(Enum):
    NoToken = 0
    Absolute1 = 1
    Absolute2 = 2
    Absolute3 = 3
    Absolute4 = 4
    Absolute5 = 5
    Last = 6
    Relative1 = 7
    Relative2 = 8
    Relative3 = 9
    Relative4 = 10


@dataclass
class Card:
    suit: Suit
    num: int

    def __repr__(self):
        return f"{{{self.suit.name} {self.num}}}"


@dataclass
class Task:
    card: Card
    token: Token = Token.NoToken
    complete: bool = False


@dataclass
class PlayerState:
    name: str
    hand: list[Card] = field(default_factory=list)
    played: Card | None = None
    collected: list[Card] = field(default_factory=list)
    tasks: list[Task] = field(default_factory=list)


@dataclass
class GameState:
    players: list[PlayerState] = field(default_factory=list)
