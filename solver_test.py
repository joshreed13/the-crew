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
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(2, Card(Suit.Blue, 3))])]
        self.assertIsNotNone(solveStep(hands, objs, 0))

    def test_singleRound_unwinnable(self):
        hands = [
            [Card(Suit.Blue, 7)],
            [Card(Suit.Magenta, 7)],
            [Card(Suit.Blue, 8)],
            [Card(Suit.Blue, 3)],
        ]
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(0, Card(Suit.Blue, 3))])]
        self.assertIsNone(solveStep(hands, objs, 0))

    def test_twoRounds_winnable(self):
        hands = [
            [Card(Suit.Blue, 7), Card(Suit.Yellow, 5)],
            [Card(Suit.Blue, 2), Card(Suit.Magenta, 7)],
            [Card(Suit.Yellow, 9), Card(Suit.Magenta, 6)],
            [Card(Suit.Blue, 3), Card(Suit.Magenta, 3)],
        ]
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(2, Card(Suit.Magenta, 3))])]
        self.assertIsNotNone(solveStep(hands, objs, 0))

    def test_twoRounds_unwinnable(self):
        hands = [
            [Card(Suit.Blue, 7), Card(Suit.Yellow, 5)],
            [Card(Suit.Blue, 2), Card(Suit.Magenta, 7)],
            [Card(Suit.Yellow, 3), Card(Suit.Magenta, 6)],
            [Card(Suit.Blue, 3), Card(Suit.Magenta, 3)],
        ]
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(2, Card(Suit.Magenta, 3))])]
        self.assertIsNone(solveStep(hands, objs, 0))

    def test_twoRounds_twoTasks(self):
        hands = [
            [Card(Suit.Blue, 7), Card(Suit.Yellow, 5)],
            [Card(Suit.Blue, 2), Card(Suit.Magenta, 7)],
            [Card(Suit.Yellow, 9), Card(Suit.Magenta, 6)],
            [Card(Suit.Blue, 3), Card(Suit.Magenta, 3)],
        ]
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(2, Card(Suit.Magenta, 3)), Task(2, Card(Suit.Magenta, 7))])]
        self.assertIsNotNone(solveStep(hands, objs, 0))

    def test_winnerLeadsNextTrick(self):
        hands = [
            [Card(Suit.Blue, 1), Card(Suit.Blue, 2)],
            [Card(Suit.Blue, 3), Card(Suit.Blue, 4)],
            [Card(Suit.Blue, 5), Card(Suit.Magenta, 1)],
            [Card(Suit.Blue, 9), Card(Suit.Green, 6)],
        ]
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(3, Card(Suit.Magenta, 1))])]
        self.assertIsNotNone(solveStep(hands, objs, 0))

    def test_complicated(self):
        hands = [
            [Card(Suit.Green, 1), Card(Suit.Yellow, 5), Card(Suit.Yellow, 8)],
            [Card(Suit.Magenta, 1), Card(Suit.Magenta, 2), Card(Suit.Magenta, 3)],
            [Card(Suit.Magenta, 4), Card(Suit.Magenta, 5), Card(Suit.Magenta, 6)],
            [Card(Suit.Green, 9), Card(Suit.Yellow, 6), Card(Suit.Blue, 7)],
        ]
        objs: list[Objective] = [TaskObjective(False, anytimeTasks=[
            Task(0, Card(Suit.Green, 9))])]
        self.assertIsNotNone(solveStep(hands, objs, 0))


class Test_generatePlays(unittest.TestCase):
    def do(self, hands: list[Hand], leadSuit: Suit | None):
        return list(generatePlays(hands, leadSuit))

    def test_basecase(self):
        self.assertEqual(self.do([], None), [Play([], [])])
        self.assertEqual(self.do([], Suit.Blue), [Play([], [])])
        self.assertEqual(self.do([], Suit.Rocket), [Play([], [])])

    def test_twoPlayers(self):
        G4 = Card(Suit.Green, 4)
        Y1 = Card(Suit.Yellow, 1)
        self.assertEqual(self.do([[G4], [Y1]], None),
                         [Play([G4, Y1], [[], []])])

    def test_twoCards(self):
        G4 = Card(Suit.Green, 4)
        Y1 = Card(Suit.Yellow, 1)
        self.assertEqual(self.do([[G4, Y1]], None),
                         [Play([G4], [[Y1]]), Play([Y1], [[G4]])])

    def test_followsSuit(self):
        G4 = Card(Suit.Green, 4)
        Y1 = Card(Suit.Yellow, 1)
        Y5 = Card(Suit.Yellow, 5)
        self.assertEqual(self.do([[G4, Y1]], Suit.Green), [Play([G4], [[Y1]])])
        self.assertEqual(self.do([[G4, Y1]], Suit.Yellow), [
                         Play([Y1], [[G4]])])
        self.assertEqual(self.do([[Y1, Y5]], Suit.Yellow), [
                         Play([Y1], [[Y5]]), Play([Y5], [[Y1]])])

    def test_singlePlayerSingleCard(self):
        G4 = Card(Suit.Green, 4)
        hand = [G4]
        self.assertEqual(self.do([hand], None), [Play([G4], [[]])])
        self.assertEqual(self.do([hand], Suit.Blue), [Play([G4], [[]])])
        self.assertEqual(self.do([hand], Suit.Green), [Play([G4], [[]])])
        self.assertEqual(self.do([hand], Suit.Rocket), [Play([G4], [[]])])

    def test_twoPlayersSingleCard(self):
        G4 = Card(Suit.Green, 4)
        G2 = Card(Suit.Green, 2)
        self.assertEqual(self.do([[G4], [G2]], None),
                         [Play([G4, G2], [[], []])])

    def test_twoPlayersTwoCards(self):
        G4 = Card(Suit.Green, 4)
        G2 = Card(Suit.Green, 2)
        G8 = Card(Suit.Green, 8)
        M3 = Card(Suit.Magenta, 3)
        self.assertEqual(self.do([[G4, G8], [G2, M3]], None), [
                         Play([G4, G2], [[G8], [M3]]), Play([G8, G2], [[G4], [M3]])])


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


class Test_applyPlayToTaskObjective(unittest.TestCase):
    def test_completedAnytimeTask(self):
        play = Play([Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)], [])
        T1 = Task(0, Card(Suit.Blue, 5))
        obj = TaskObjective(False, anytimeTasks=[T1])
        self.assertEqual(applyPlayToTaskObjective(obj, play, 0), True)

    def test_notCompleteAnytimeTask(self):
        play = Play([Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)], [])
        T1 = Task(0, Card(Suit.Blue, 6))
        obj = TaskObjective(False, anytimeTasks=[T1])
        self.assertEqual(applyPlayToTaskObjective(obj, play, 0), obj)

    def test_failedAnytimeTask(self):
        play = Play([Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)], [])
        T1 = Task(1, Card(Suit.Blue, 5))
        obj = TaskObjective(False, anytimeTasks=[T1])
        self.assertEqual(applyPlayToTaskObjective(obj, play, 0), False)

    def test_twoAnytimeTasks(self):
        play = Play([Card(Suit.Blue, 3), Card(
            Suit.Green, 7), Card(Suit.Blue, 5)], [])
        T1 = Task(0, Card(Suit.Blue, 5))
        T2 = Task(0, Card(Suit.Blue, 8))
        obj = TaskObjective(False, anytimeTasks=[T1, T2])
        self.assertEqual(applyPlayToTaskObjective(obj, play, 0),
                         TaskObjective(False, anytimeTasks=[T2]))
        obj = TaskObjective(False, anytimeTasks=[T2, T1])
        self.assertEqual(applyPlayToTaskObjective(obj, play, 0),
                         TaskObjective(False, anytimeTasks=[T2]))


class Test_rotateToIndex(unittest.TestCase):
    def test_rotateToIndex(self):
        B1 = Card(Suit.Blue, 1)
        B2 = Card(Suit.Blue, 2)
        B3 = Card(Suit.Blue, 3)
        B4 = Card(Suit.Blue, 4)
        self.assertEqual(rotateToIndex([B1, B2, B3, B4], 0), [B1, B2, B3, B4])
        self.assertEqual(rotateToIndex([B1, B2, B3, B4], 1), [B2, B3, B4, B1])
        self.assertEqual(rotateToIndex([B1, B2, B3, B4], 2), [B3, B4, B1, B2])
        self.assertEqual(rotateToIndex([B1, B2, B3, B4], 3), [B4, B1, B2, B3])
