from model import *
from typing import Iterator, TypeVar

Play = list[tuple[Card, list[Card]]]
T = TypeVar('T')


def solve(state: GameState) -> list[Play] | None:
    hands = [p.hand for p in state.players]
    tasks = [p.tasks for p in state.players]
    return solveStep(hands, tasks)


def solveStep(hands: list[list[Card]], tasks: list[list[Task]]) -> list[Play] | None:
    for play in generatePlays(hands, None):
        playedCards = [card for card, remaining in play]
        winnerIndex = playedCards.index(getTrickWinner(playedCards))

        remainingTasks = getRemainingTasks(winnerIndex, playedCards, tasks)

        if remainingTasks is None:
            continue

        if not any(t for t in remainingTasks):
            return [play]

        remainingHands = [remaining for card, remaining in play]
        result = solveStep(rotateToIndex(remainingHands, winnerIndex),
                           rotateToIndex(remainingTasks, winnerIndex))
        if result:
            return [play] + result
    return None


def generatePlays(hands: list[list[Card]], leadSuit: Suit | None) -> Iterator[Play]:
    if not hands:
        yield []
        return

    holdingLeadSuit = (leadSuit is not None) and any(
        c.suit == leadSuit for c in hands[0])

    for card in hands[0]:
        if not holdingLeadSuit or card.suit == leadSuit:
            pickedSuit = leadSuit if leadSuit is not None else card.suit
            remainingCards = [c for c in hands[0] if c != card]
            for subplay in generatePlays(hands[1:], pickedSuit):
                yield [(card, remainingCards)] + subplay


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


def getRemainingTasks(winnerIndex: int, playedCards: list[Card], tasks: list[list[Task]]) -> list[list[Task]] | None:
    newTasks = [[] for _ in tasks]
    for i, taskList in enumerate(tasks):
        for task in taskList:
            if task.card in playedCards:
                if i == winnerIndex:
                    pass  # Task Complete
                else:
                    return None
            else:
                newTasks[i].append(task)
    return newTasks


def rotateToIndex(hands: list[T], newLeaderIndex: int) -> list[T]:
    return hands[newLeaderIndex:] + hands[0:newLeaderIndex]
