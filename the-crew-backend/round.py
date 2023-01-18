from dataclasses import dataclass
from typing import Optional


@dataclass
class Card:
    suit: str
    value: int


@dataclass
class PlayerState:
    name: str
    hand: list[Card]


@dataclass
class Task:
    type: str
    order: int
    card: Optional[Card]
    playerNum: Optional[int]


@dataclass
class Trick:
    turns: list[Optional[Card]]


class Round:
    taskId: int
    players: list[PlayerState]
    objectives: dict[int, Task]
    tricks: list[Trick]

    def __init__(self):
        self.taskId = 1
        self.players = [
            PlayerState("Player 1", [Card("B", 1)]),
            PlayerState("Player 2", [Card("Y", 2)]),
            PlayerState("Player 3", [Card("M", 3)]),
            PlayerState("Player 4", [Card("G", 4)])
        ]
        self.objectives = {}
        self.tricks = [Trick([None, None, None, None])]

    def setPlayerName(self, playerNum: int, name: str):
        self.players[playerNum].name = name

    def setPlayerHand(self, playerNum: int, cards: list[Card]):
        self.players[playerNum].hand = cards

    def addObjective(self, objtype: str, order: int, card: Optional[Card], playerNum: Optional[int]):
        self.objectives[self.taskId] = Task(
            objtype, order, card, playerNum)
        self.taskId += 1

    def removeObjective(self, id: int):
        del self.objectives[id]

    def setObjectiveCard(self, id: int, card: Card):
        self.objectives[id].card = card

    def setObjectivePlayer(self, id: int, playerNum: int):
        self.objectives[id].playerNum = playerNum

    def setTrickTurnCard(self, trickIndex: int, turnIndex: int, card: Card):
        self.tricks[trickIndex].turns[turnIndex] = card

    def toJson(self):
        def toPlayer(playerNum: int, player: PlayerState):
            return {
                "num": playerNum,
                "name": player.name,
                "isCommander": Card("R", 4) in player.hand
            }

        def toCard(card: Card):
            return {
                "suit": card.suit,
                "value": card.value
            }

        def toTask(id, task: Task):
            return {
                "id": id,
                "type": task.type,
                "order": task.order,
                "card": toCard(task.card) if task.card is not None else None,
                "player": toPlayer(task.playerNum, self.players[task.playerNum]) if task.playerNum is not None else None,
            }

        tricks = [{
            "turns": [{
                "player": toPlayer(playerNum, self.players[playerNum]),
                "card": toCard(card)if card is not None else None,
                "isLeader": False,
                "isWinner": False,
                "isNextToPlay": False,
            } for playerNum, card in enumerate(trick.turns)]
        } for trick in self.tricks]

        return {
            "handPage": {
                "heldCards": [[toCard(card) for card in player.hand] for player in self.players]
            },
            "objectivePage": {
                "tasks": [toTask(id, task) for id, task in self.objectives.items()],
                "nextAbsolute": 1 + max((task.order for task in self.objectives.values() if task.type == "absolute"), default=0),
                "nextRelative": 1 + max((task.order for task in self.objectives.values() if task.type == "relative"), default=0),
                "haveLast": any(task.type == "last" for task in self.objectives.values()),
                "players": [toPlayer(playerNum, player) for playerNum, player in enumerate(self.players)],
            },
            "tricksPage": {
                "tricks": tricks
            },
            "controlPanel": {
                "players": [{
                    "player": toPlayer(playerNum, player),
                    "hand": [toCard(card) for card in player.hand],
                    "tasks": [toTask(id, task) for id, task in self.objectives.items() if task.playerNum == playerNum],
                } for playerNum, player in enumerate(self.players)],
                "tricks": tricks
            },
        }
