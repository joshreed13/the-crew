import itertools

from model import *


def newGame(players: list[PlayerState]) -> GameState:
    return GameState(sortCommanderToTop(players))


def sortCommanderToTop(players: list[PlayerState]) -> list[PlayerState]:
    return list(itertools.islice(itertools.dropwhile(lambda p: not isCommander(p), itertools.cycle(players)), len(players)))


def isCommander(player: PlayerState) -> bool:
    return Card(Suit.Rocket, 4) in player.hand


def playCard(state: GameState, card: Card):
    for player in state.players:
        if hasPlayed(player):
            continue
        else:
            assert card in player.hand
            player.hand.remove(card)
            player.played = card

    if all(state.players, hasPlayed):
        processTrick(state)


def hasPlayed(player: PlayerState) -> bool:
    return player.played is not None


def processTrick(state: GameState):
    cardsPlayed = [p.played for p in state.players]
    winner = getTrickWinner(cardsPlayed)
    for player in state.players:
        if player.played == winner:
            evaluateTasks(player, cardsPlayed)
            player.collected += cardsPlayed
        player.played = None


def getWinnerOfSuit(cards: list[Card], suit: Suit) -> Card | None:
    relevantCards = [c for c in cards if c.suit == suit]
    if relevantCards:
        return max(relevantCards, lambda c: c.num)
    else:
        return None


def getTrickWinner(cards: list[Card]) -> Card:
    trumpWinner = getWinnerOfSuit(cards, Suit.Rocket)
    if trumpWinner is not None:
        return trumpWinner
    else:
        leadSuit = cards[0].suit
        return getWinnerOfSuit(cards, leadSuit)


def evaluateTasks(player: PlayerState, cards: list[Card]):
    for task in player.tasks:
        if task.card in cards:
            task.complete = True
            # TODO Check ordering of task


"""
Normal Game:
Get PlayerState (hands, objectives) from every player
Assemble PlayerState values into GameState
    Validate state
    Determine starting player
Record each card played (from hand to played)
After all players
    Score trick
    Move cards to collected
    Update objectives
    Update starting player
    Run solver?

Other Processes:
Swap cards
Set state in middle of game

"""
