import logging
import os
import threading

import requests

from round import Round

SOLVER_ENDPOINT = os.environ.get('SOLVER_ENDPOINT')


LOGGER = logging.getLogger("solver")


class Solver:
    def __init__(self):
        LOGGER.info(f"Init solver interface: {SOLVER_ENDPOINT}")
        self.nextId = 0

    def solve(self, state: Round, callback):
        if SOLVER_ENDPOINT is None:
            return

        data = self._stateToInput(state)
        threading.Thread(target=sendRequest, args=(
            self.nextId, data, callback)).start()
        self.nextId += 1

    def _stateToInput(self, state: Round):
        def cardToStr(card):
            return f"{card.suit}{card.value}" if card is not None else None

        return {
            "hands": [[
                cardToStr(card) for card in player.hand
            ]for player in state.players],
            "tasks": [{
                "task_type": task.type,
                "order": task.order,
                "card": cardToStr(task.card),
                "player_num": task.playerNum,
            } for id, task in state.objectives.items()],
            "curr_leader": state.tricks[-1].leadPlayerNum
        }


def sendRequest(id, data, callback):
    if SOLVER_ENDPOINT is None:
        return
    LOGGER.info(f"Requesting solve: {data}")
    response = requests.post(SOLVER_ENDPOINT, json=data)
    LOGGER.info(f"Response received: {response.text}")
    callback(id, response.json())
