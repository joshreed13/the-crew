from dataclasses import dataclass
from model import *
from typing import Iterator, TypeVar

T = TypeVar('T')
Hand = list[Card]

SOLVES = 0


@dataclass
class Play:
    playedCards: list[Card]
    remainingHands: list[Hand]


def solve(state: GameState) -> list[Play] | None:
    hands = [p.hand for p in state.players]
    objectives = state.objectives
    leader = state.currentLeader
    return solveStep(hands, objectives, leader)


def solveStep(hands: list[Hand], objectives: list[Objective], leader: PlayerIndex) -> list[Play] | None:
    global SOLVES
    SOLVES += 1
    for play in generatePlays(rotateToIndex(hands, leader), None):
        result = solvePlay(play, len(hands), objectives, leader)
        if result is not None:
            return result
    return None


def solvePlay(play: Play, numPlayers: int, objectives: list[Objective], leader: PlayerIndex) -> list[Play] | None:
    winnerOffset = play.playedCards.index(getTrickWinner(play.playedCards))
    winner = (leader + winnerOffset) % numPlayers

    outObjs = [applyPlayToObj(obj, play, winner) for obj in objectives]
    newObjectives = [x for x in outObjs if isinstance(x, Objective)]

    if not all(bool(x) for x in outObjs):  # An objective has failed
        return None
    elif not newObjectives:  # No more objectives
        return [play]
    else:
        newHands = rotateToIndex(play.remainingHands, numPlayers - leader)
        result = solveStep(newHands, newObjectives, winner)
        if result:
            return [play] + result
        else:
            return None


def generatePlays(hands: list[Hand], leadSuit: Suit | None) -> Iterator[Play]:
    if not hands:
        yield Play([], [])
        return

    hand = hands[0]

    holdingLeadSuit = (leadSuit is not None) and any(
        c.suit == leadSuit for c in hand)

    for card in hand:
        if not holdingLeadSuit or card.suit == leadSuit:
            newLeadSuit = leadSuit if leadSuit is not None else card.suit
            remainingHand = [c for c in hand if c != card]
            for subplay in generatePlays(hands[1:], newLeadSuit):
                yield Play([card] + subplay.playedCards, [remainingHand] + subplay.remainingHands)


def getWinnerOfSuit(cards: list[Card], suit: Suit) -> Card | None:
    relevantCards = [c for c in cards if c.suit == suit]
    if relevantCards:
        return max(relevantCards, key=lambda c: c.num)
    else:
        return None


def getTrickWinner(cards: list[Card]) -> Card:
    assert cards
    trumpWinner = getWinnerOfSuit(cards, Suit.Rocket)
    if trumpWinner is not None:
        return trumpWinner
    else:
        leadSuit = cards[0].suit
        result = getWinnerOfSuit(cards, leadSuit)
        assert result is not None
        return result


def rotateToIndex(hands: list[T], newLeaderIndex: int) -> list[T]:
    return hands[newLeaderIndex:] + hands[0:newLeaderIndex]


def applyPlayToObj(objective: Objective, play: Play, winner: PlayerIndex) -> Objective | bool:
    if isinstance(objective, TaskObjective):
        return applyPlayToTaskObjective(objective, play, winner)
    else:
        raise ValueError("Unsuported objective type")


def applyPlayToTaskObjective(objective: TaskObjective, play: Play, winner: PlayerIndex) -> Objective | bool:
    noMoreAll = False
    newAbsolute = []
    for task in objective.absoluteTasks:
        if task.card in play.playedCards:
            if noMoreAll:
                return False
            elif winner == task.player:
                pass
            else:
                return False
        else:
            noMoreAll = True
            newAbsolute.append(task)

    noMoreRelative = False
    newRelative = []
    for task in objective.relativeTasks:
        if task.card in play.playedCards:
            if noMoreAll or noMoreRelative:
                return False
            elif winner == task.player:
                pass
            else:
                return False
        else:
            noMoreRelative = True
            newRelative.append(task)

    newAnytime = []
    for task in objective.anytimeTasks:
        if task.card in play.playedCards:
            if noMoreAll:
                return False
            elif winner == task.player:
                pass
            else:
                return False
        else:
            newAnytime.append(task)

    if objective.lastTask is not None:
        if objective.lastTask in play.playedCards:
            if objective.absoluteTasks or objective.relativeTasks or objective.anytimeTasks:
                return False
            elif winner == objective.lastTask:
                return True
            else:
                return False

    if newAbsolute or newRelative or newAnytime or objective.lastTask:
        return TaskObjective(False, newAbsolute, newRelative, newAnytime, objective.lastTask)
    else:
        return True
