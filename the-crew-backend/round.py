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
    leadPlayerNum: int
    nextTurnPlayerNum: Optional[int]
    winnerPlayerNum: Optional[int] = None


@dataclass
class Solve:
    id: int
    success: bool
    result: bool
    duration: int


COMMANDER_CARD = Card("R", 4)


class Round:
    taskId: int
    players: list[PlayerState]
    objectives: dict[int, Task]
    tricks: list[Trick]
    solves: list[Solve]

    def __init__(self):
        self.reset()

    def reset(self):
        self.taskId = 1
        self.players = [
            PlayerState("Player 1", [Card("B", 1)]),
            PlayerState("Player 2", [Card("Y", 2)]),
            PlayerState("Player 3", [Card("M", 3)]),
            PlayerState("Player 4", [Card("G", 4)])
        ]
        self.objectives = {}
        self.tricks = [Trick([None, None, None, None],
                             leadPlayerNum=0, nextTurnPlayerNum=0)]
        self.solves = []

    def setPlayerName(self, playerNum: int, name: str):
        self.players[playerNum].name = name

    def setPlayerHand(self, playerNum: int, cards: list[Card]):
        self.players[playerNum].hand = cards

        if COMMANDER_CARD in cards and len(self.tricks) == 1:
            firstTrick = self.tricks[0]
            if all(c is None for c in firstTrick.turns):
                firstTrick.leadPlayerNum = playerNum
                firstTrick.nextTurnPlayerNum = _trickNextPlayer(firstTrick)

    def addObjective(self, objtype: str, order: int, card: Optional[Card], playerNum: Optional[int]):
        self.objectives[self.taskId] = Task(objtype, order, card, playerNum)
        self.taskId += 1

    def removeObjective(self, id: int):
        obj = self.objectives.get(id)
        if obj is not None and obj.type in {"absolute", "relative"}:
            for _, task in self.objectives.items():
                if task.type == obj.type and task.order > obj.order:
                    task.order -= 1
        del self.objectives[id]

    def setObjectiveCard(self, id: int, card: Card):
        self.objectives[id].card = card

    def setObjectivePlayer(self, id: int, playerNum: int):
        self.objectives[id].playerNum = playerNum

    def setTrickTurnCard(self, trickIndex: int, turnIndex: int, card: Card):
        trick = self.tricks[trickIndex]

        alreadyPlayed = trick.turns[turnIndex]
        if alreadyPlayed is not None:
            self.players[turnIndex].hand.append(alreadyPlayed)
        trick.turns[turnIndex] = card
        try:
            self.players[turnIndex].hand.remove(card)
        except ValueError:
            pass

        trick.nextTurnPlayerNum = _trickNextPlayer(trick)
        if trick.nextTurnPlayerNum is None:
            winner = _trickWinner(
                cast(list[Card], trick.turns), cast(Card, trick.turns[trick.leadPlayerNum]))
            trick.winnerPlayerNum = winner
            self.tricks.append(
                Trick([None, None, None, None], leadPlayerNum=winner, nextTurnPlayerNum=winner))
        else:
            trick.winnerPlayerNum = None

    def addSolverResult(self, id, result):
        self.solves.append(
            Solve(id, result["success"], result["result"], result["duration"]))

    def toJson(self):
        def toPlayer(playerNum: int, player: PlayerState):
            return {
                "num": playerNum,
                "name": player.name,
                "isCommander": COMMANDER_CARD in player.hand
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
                "tricks": tricks,
                "heldCards": [[toCard(card) for card in player.hand] for player in self.players]
            },
            "controlPanel": {
                "players": [{
                    "player": toPlayer(playerNum, player),
                    "hand": [toCard(card) for card in player.hand],
                    "tasks": [toTask(id, task) for id, task in self.objectives.items() if task.playerNum == playerNum],
                } for playerNum, player in enumerate(self.players)]
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


def _trickNextPlayer(trick: Trick) -> Optional[int]:
    plays = trick.turns[trick.leadPlayerNum:] + \
        trick.turns[:trick.leadPlayerNum]
    try:
        firstNonePlay = [card is None for card in plays].index(True)
        return (firstNonePlay + trick.leadPlayerNum) % len(trick.turns)
    except ValueError:
        return None


def _trickWinner(cards: list[Card], leadCard: Card) -> int:
    assert cards
    eligible = [card for card in cards if card.suit == "R"]
    if not eligible:
        eligible = [card for card in cards if card.suit == leadCard.suit]
    winningCard = max(eligible, key=lambda c: c.value)
    return cards.index(winningCard)
