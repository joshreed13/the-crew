from dataclasses import dataclass
from enum import Enum


@Enum
class Suit:
    Blue = 0
    Yellow = 1
    Magenta = 2
    Green = 3
    Rocket = 4


@Enum
class Token:
    Absolute1 = 0
    Absolute2 = 1
    Absolute3 = 2
    Absolute4 = 3
    Absolute5 = 4
    Last = 5
    Relative1 = 6
    Relative2 = 7
    Relative3 = 8
    Relative4 = 9


@dataclass
class Card:
    suit: Suit
    num: int


@dataclass
class Task:
    card: Card
    token: Token
    complete: bool


@dataclass
class PlayerState:
    name: str
    hand: list[Card]
    played: Card | None
    collected: list[Card]
    tasks: list[Task]


@dataclass
class GameState:
    players: list[PlayerState]
