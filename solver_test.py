import unittest

from solver import *
from model import *


class Test_solveStep(unittest.TestCase):
    def test_singleRound_winnable(self):
        hands = [
            [Card(Suit.Blue, 7)],
            [Card(Suit.Magenta, 7)],
            [Card(Suit.Blue, 8)],
            [Card(Suit.Blue, 3)],
        ]
        tasks = [
            [],
            [],
            [Task(Card(Suit.Blue, 3), Token.NoToken)],
            [],
        ]
        self.assertTrue(solveStep(hands, tasks))

    def test_singleRound_unwinnable(self):
        hands = [
            [Card(Suit.Blue, 7)],
            [Card(Suit.Magenta, 7)],
            [Card(Suit.Blue, 8)],
            [Card(Suit.Blue, 3)],
        ]
        tasks = [
            [Task(Card(Suit.Blue, 3), Token.NoToken)],
            [],
            [],
            [],
        ]
        self.assertFalse(solveStep(hands, tasks))

    def test_twoRounds_winnable(self):
        hands = [
            [Card(Suit.Blue, 7), Card(Suit.Yellow, 5)],
            [Card(Suit.Blue, 2), Card(Suit.Magenta, 7)],
            [Card(Suit.Yellow, 9), Card(Suit.Magenta, 6)],
            [Card(Suit.Blue, 3), Card(Suit.Magenta, 3)],
        ]
        tasks = [
            [],
            [],
            [Task(Card(Suit.Magenta, 3), Token.NoToken)],
            [],
        ]
        self.assertTrue(solveStep(hands, tasks))

    def test_twoRounds_unwinnable(self):
        hands = [
            [Card(Suit.Blue, 7), Card(Suit.Yellow, 5)],
            [Card(Suit.Blue, 2), Card(Suit.Magenta, 7)],
            [Card(Suit.Yellow, 3), Card(Suit.Magenta, 6)],
            [Card(Suit.Blue, 3), Card(Suit.Magenta, 3)],
        ]
        tasks = [
            [],
            [],
            [Task(Card(Suit.Magenta, 3), Token.NoToken)],
            [],
        ]
        self.assertFalse(solveStep(hands, tasks))

    def test_twoRounds_twoTasks(self):
        hands = [
            [Card(Suit.Blue, 7), Card(Suit.Yellow, 5)],
            [Card(Suit.Blue, 2), Card(Suit.Magenta, 7)],
            [Card(Suit.Yellow, 9), Card(Suit.Magenta, 6)],
            [Card(Suit.Blue, 3), Card(Suit.Magenta, 3)],
        ]
        tasks = [
            [],
            [],
            [Task(Card(Suit.Magenta, 3), Token.NoToken),
             Task(Card(Suit.Magenta, 7), Token.NoToken)],
            [],
        ]
        self.assertTrue(solveStep(hands, tasks))


class Test_generatePlays(unittest.TestCase):
    def do(self, hands: list[list[Card]], leadSuit: Suit | None):
        return list(generatePlays(hands, leadSuit))

    def test_basecase(self):
        self.assertEqual(self.do([], None), [[]])
        self.assertEqual(self.do([], Suit.Blue), [[]])
        self.assertEqual(self.do([], Suit.Rocket), [[]])

    def test_twoPlayers(self):
        G4 = Card(Suit.Green, 4)
        Y1 = Card(Suit.Yellow, 1)
        self.assertEqual(self.do([[G4], [Y1]], None), [[(G4, []), (Y1, [])]])

    def test_twoCards(self):
        G4 = Card(Suit.Green, 4)
        Y1 = Card(Suit.Yellow, 1)
        result = self.do([[G4, Y1]], None)
        expected = [[(G4, [Y1])], [(Y1, [G4])]]
        self.assertEqual(result, expected)

    def test_followsSuit(self):
        G4 = Card(Suit.Green, 4)
        Y1 = Card(Suit.Yellow, 1)
        Y5 = Card(Suit.Yellow, 5)
        self.assertEqual(self.do([[G4, Y1]], Suit.Green), [[(G4, [Y1])]])
        self.assertEqual(self.do([[G4, Y1]], Suit.Yellow), [[(Y1, [G4])]])
        self.assertEqual(self.do([[Y1, Y5]], Suit.Yellow), [
                         [(Y1, [Y5])], [(Y5, [Y1])]])

    def test_singlePlayerSingleCard(self):
        G4 = Card(Suit.Green, 4)
        hand = [G4]
        self.assertEqual(self.do([hand], None), [[(G4, [])]])
        self.assertEqual(self.do([hand], Suit.Blue), [[(G4, [])]])
        self.assertEqual(self.do([hand], Suit.Green), [[(G4, [])]])
        self.assertEqual(self.do([hand], Suit.Rocket), [[(G4, [])]])

    def test_twoPlayersSingleCard(self):
        G4 = Card(Suit.Green, 4)
        G2 = Card(Suit.Green, 2)
        self.assertEqual(self.do([[G4], [G2]], None), [[(G4, []), (G2, [])]])

    def test_twoPlayersTwoCards(self):
        G4 = Card(Suit.Green, 4)
        G2 = Card(Suit.Green, 2)
        G8 = Card(Suit.Green, 8)
        M3 = Card(Suit.Magenta, 3)
        self.assertEqual(self.do([[G4, G8], [G2, M3]], None), [
                         [(G4, [G8]), (G2, [M3])], [(G8, [G4]), (G2, [M3])]])


class Test_getWinnerOfSuit(unittest.TestCase):
    def test_empty(self):
        self.assertEqual(getWinnerOfSuit([], Suit.Blue), None)

    def test_singleCard(self):
        self.assertEqual(getWinnerOfSuit(
            [Card(Suit.Blue, 8)], Suit.Blue), Card(Suit.Blue, 8))

    def test_higherCardWins(self):
        self.assertEqual(getWinnerOfSuit(
            [Card(Suit.Blue, 8), Card(Suit.Blue, 6)], Suit.Blue), Card(Suit.Blue, 8))
        self.assertEqual(getWinnerOfSuit(
            [Card(Suit.Blue, 6), Card(Suit.Blue, 8)], Suit.Blue), Card(Suit.Blue, 8))

    def test_offsuitIsIgnored(self):
        self.assertEqual(getWinnerOfSuit(
            [Card(Suit.Blue, 8), Card(Suit.Green, 9)], Suit.Blue), Card(Suit.Blue, 8))

    def test_missingSuit(self):
        self.assertEqual(getWinnerOfSuit(
            [Card(Suit.Blue, 8)], Suit.Green), None)
        self.assertEqual(getWinnerOfSuit([Card(Suit.Blue, 8), Card(Suit.Green, 7), Card(
            Suit.Magenta, 3), Card(Suit.Yellow, 1)], Suit.Rocket), None)

    def test_multipleCards(self):
        self.assertEqual(getWinnerOfSuit([Card(Suit.Blue, 8), Card(
            Suit.Blue, 7), Card(Suit.Green, 9)], Suit.Blue), Card(Suit.Blue, 8))

    def test_trumpIsNotSpecial(self):
        self.assertEqual(getWinnerOfSuit([Card(Suit.Blue, 1), Card(
            Suit.Rocket, 4)], Suit.Blue), Card(Suit.Blue, 1))


class Test_getTrickWinner(unittest.TestCase):
    def test_singleCard(self):
        self.assertEqual(getTrickWinner(
            [Card(Suit.Blue, 4)]), Card(Suit.Blue, 4))

    def test_sameSuit(self):
        self.assertEqual(getTrickWinner(
            [Card(Suit.Blue, 4), Card(Suit.Blue, 7)]), Card(Suit.Blue, 7))
        self.assertEqual(getTrickWinner(
            [Card(Suit.Blue, 7), Card(Suit.Blue, 4)]), Card(Suit.Blue, 7))

    def test_offsuitIsIgnored(self):
        self.assertEqual(getTrickWinner(
            [Card(Suit.Blue, 4), Card(Suit.Green, 7)]), Card(Suit.Blue, 4))

    def test_trumpWins(self):
        self.assertEqual(getTrickWinner(
            [Card(Suit.Blue, 4), Card(Suit.Rocket, 2)]), Card(Suit.Rocket, 2))

    def test_multipleTrump(self):
        self.assertEqual(getTrickWinner(
            [Card(Suit.Blue, 4), Card(Suit.Rocket, 2), Card(Suit.Rocket, 4)]), Card(Suit.Rocket, 4))

    def test_leadTrump(self):
        self.assertEqual(getTrickWinner(
            [Card(Suit.Rocket, 2), Card(Suit.Rocket, 4), Card(Suit.Blue, 4)]), Card(Suit.Rocket, 4))


class Test_getRemainingTasks(unittest.TestCase):
    def test_completedTask(self):
        cardsPlayed = [Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)]
        T1 = Task(Card(Suit.Blue, 5), Token.NoToken)
        tasks = [[T1], [], []]
        self.assertEqual(getRemainingTasks(
            0, cardsPlayed, tasks), [[], [], []])

    def test_notComplete(self):
        cardsPlayed = [Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)]
        T1 = Task(Card(Suit.Blue, 6), Token.NoToken)
        tasks = [[T1], [], []]
        self.assertEqual(getRemainingTasks(
            0, cardsPlayed, tasks), [[T1], [], []])

    def test_failedTask(self):
        cardsPlayed = [Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)]
        T1 = Task(Card(Suit.Blue, 5), Token.NoToken)
        tasks = [[], [T1], []]
        self.assertEqual(getRemainingTasks(0, cardsPlayed, tasks), None)

    def test_twoTasks(self):
        cardsPlayed = [Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)]
        T1 = Task(Card(Suit.Blue, 5), Token.NoToken)
        T2 = Task(Card(Suit.Blue, 8), Token.NoToken)
        self.assertEqual(getRemainingTasks(0, cardsPlayed,
                                           [[T1, T2], [], []]), [[T2], [], []])
        self.assertEqual(getRemainingTasks(0, cardsPlayed,
                                           [[T2, T1], [], []]), [[T2], [], []])
        self.assertEqual(getRemainingTasks(0, cardsPlayed,
                                           [[T1], [T2], []]), [[], [T2], []])
