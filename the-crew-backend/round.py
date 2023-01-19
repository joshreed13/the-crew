from dataclasses import dataclass
from typing import Optional, cast


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
    leadPlayerNum: int = 0
    winnerPlayerNum: Optional[int] = None
    nextTurnPlayerNum: Optional[int] = None


@dataclass
class Solve:
    id: int
    success: bool
    result: bool
    duration: int


class Round:
    taskId: int
    players: list[PlayerState]
    objectives: dict[int, Task]
    tricks: list[Trick]
    solves: list[Solve]

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
        self.solves = []

    def setPlayerName(self, playerNum: int, name: str):
        self.players[playerNum].name = name

    def setPlayerHand(self, playerNum: int, cards: list[Card]):
        self.players[playerNum].hand = cards

    def addObjective(self, objtype: str, order: int, card: Optional[Card], playerNum: Optional[int]):
        self.objectives[self.taskId] = Task(
            objtype, order, card, playerNum)
        self.taskId += 1

    def removeObjective(self, id: int):
        obj = self.objectives.get(id)
        if obj and obj.type in {"absolute", "relative"}:
            for id, task in self.objectives.items():
                if task.type == obj.type and task.order > obj.order:
                    task.order -= 1
        del self.objectives[id]

    def setObjectiveCard(self, id: int, card: Card):
        self.objectives[id].card = card

    def setObjectivePlayer(self, id: int, playerNum: int):
        self.objectives[id].playerNum = playerNum

    def setTrickTurnCard(self, trickIndex: int, turnIndex: int, card: Card):
        trick = self.tricks[trickIndex]
        trick.turns[turnIndex] = card

        try:
            trick.nextTurnPlayerNum = [
                card is None for card in trick.turns].index(True)
            trick.winnerPlayerNum = None
        except ValueError:
            trick.nextTurnPlayerNum = None
            trick.winnerPlayerNum = _trickWinner(cast(list[Card], trick.turns))

    def addSolverResult(self, id, result):
        self.solves.append(
            Solve(id, result["success"], result["result"], result["duration"]))

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
                "isLeader": playerNum == trick.leadPlayerNum,
                "isWinner": playerNum == trick.winnerPlayerNum,
                "isNextToPlay": playerNum == trick.nextTurnPlayerNum,
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
            "solverPage": {
                "solves": [{
                    "id": solve.id,
                    "success": solve.success,
                    "result": solve.result,
                    "duration": solve.duration,
                } for solve in self.solves]
            }
        }


def _trickWinner(cards: list[Card]):
    assert cards
    eligible = [card for card in cards if card.suit == "R"]
    if not eligible:
        leadSuit = cards[0].suit
        eligible = [card for card in cards if card.suit == leadSuit]
    winningCard = max(eligible, key=lambda c: c.value)
    return cards.index(winningCard)
