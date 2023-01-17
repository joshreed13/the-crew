from flask import Flask, request, Response, jsonify
import werkzeug.exceptions
from flask_socketio import SocketIO, emit
from model import *

app = Flask(__name__)
socketio = SocketIO(app)

TASKID = 1
STATE: RoundState = RoundState(
    players=[
        PlayerState("Player 1", [Card("B", 1)]),
        PlayerState("Player 2", [Card("Y", 2)]),
        PlayerState("Player 3", [Card("M", 3)]),
        PlayerState("Player 4", [Card("G", 4)])
    ],
    objectives={},
    tricks=[Trick([None, None, None, None])],
)


@socketio.on('connect')
def handle_connect(auth):
    print("New client connected")
    emit("appstate", buildStateJson())


def publishUpdate():
    socketio.emit("appstate", buildStateJson())


@app.route("/api/appstate", methods=['GET'])
def get_state():
    return jsonify(buildStateJson())


def buildStateJson():
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
            "player": toPlayer(task.playerNum, STATE.players[task.playerNum]) if task.playerNum is not None else None,
        }

    tricks = [{
        "turns": [{
            "player": toPlayer(playerNum, STATE.players[playerNum]),
            "card": toCard(card)if card is not None else None,
            "isLeader": False,
            "isWinner": False,
            "isNextToPlay": False,
        } for playerNum, card in enumerate(trick.turns)]
    } for trick in STATE.tricks]

    return {
        "handPage": {
            "heldCards": [[toCard(card) for card in player.hand] for player in STATE.players]
        },
        "objectivePage": {
            "tasks": [toTask(id, task) for id, task in STATE.objectives.items()],
            "nextAbsolute": 1 + max((task.order for task in STATE.objectives.values() if task.type == "absolute"), default=0),
            "nextRelative": 1 + max((task.order for task in STATE.objectives.values() if task.type == "relative"), default=0),
            "haveLast": any(task.type == "last" for task in STATE.objectives.values()),
            "players": [toPlayer(playerNum, player) for playerNum, player in enumerate(STATE.players)],
        },
        "tricksPage": {
            "tricks": tricks
        },
        "controlPanel": {
            "players": [{
                "player": toPlayer(playerNum, player),
                "hand": [toCard(card) for card in player.hand],
                "tasks": [toTask(id, task) for id, task in STATE.objectives.items() if task.playerNum == playerNum],
            } for playerNum, player in enumerate(STATE.players)],
            "tricks": tricks
        },
    }


@ app.route("/api/player/<int:playerNum>/name", methods=['PUT'])
def setPlayerName(playerNum):
    validateRequestIsJson(request)
    playerNum = parsePlayerNum(playerNum)

    name = request.get_json()["name"]
    STATE.players[playerNum].name = name
    publishUpdate()

    return Response("Success")


@ app.route("/api/player/<int:playerNum>/hand", methods=['PUT'])
def setPlayerHand(playerNum):
    validateRequestIsJson(request)
    playerNum = parsePlayerNum(playerNum)

    cards = parseCards(request.get_json()["cards"])
    STATE.players[playerNum].hand = cards
    publishUpdate()

    return Response("Success")


@ app.route("/api/objective/add", methods=['POST'])
def addObjective():
    global TASKID
    validateRequestIsJson(request)

    objtype = request.get_json()["type"]
    validateObjectiveType(objtype)
    order = int(request.get_json()["order"])
    validateObjectiveOrder(objtype, order)
    card = request.get_json().get("card")
    if card is not None:
        card = parseCard(card)
    playerNum = request.get_json().get("playerNum")
    if playerNum is not None:
        playerNum = parsePlayerNum(playerNum)

    STATE.objectives[TASKID] = Task(objtype, order, card, playerNum)
    TASKID += 1
    publishUpdate()

    return Response("Success")


@ app.route("/api/objective/<int:id>", methods=['DELETE'])
def removeObjective(id):
    validateTaskId(id)

    del STATE.objectives[id]
    publishUpdate()

    return Response("Success")


@ app.route("/api/objective/<int:id>/card", methods=['PUT'])
def setObjectiveCard(id):
    validateRequestIsJson(request)
    validateTaskId(id)

    card = parseCard(request.get_json()["card"])
    STATE.objectives[id].card = card
    publishUpdate()

    return Response("Success")


@ app.route("/api/objective/<int:id>/player", methods=['PUT'])
def setObjectivePlayer(id):
    validateRequestIsJson(request)
    validateTaskId(id)

    playerNum = parsePlayerNum(request.get_json()["playerNum"])
    STATE.objectives[id].playerNum = playerNum
    publishUpdate()

    return Response("Success")


@ app.route("/api/trick/<int:trickIndex>/<int:turnIndex>/card", methods=['PUT'])
def setTrickTurnCard(trickIndex, turnIndex):
    validateRequestIsJson(request)
    validateTrickIndex(trickIndex)
    validateTurnIndex(trickIndex, turnIndex)

    card = parseCard(request.get_json()["card"])
    STATE.tricks[trickIndex].turns[turnIndex] = card
    publishUpdate()

    return Response("Success")


def validateRequestIsJson(request):
    if not request.is_json:
        raise werkzeug.exceptions.UnsupportedMediaType("Expected JSON")


def validatePlayerNum(playerNum):
    if not (0 <= playerNum < 4):
        raise werkzeug.exceptions.BadRequest("Bad player number")


def validateCards(cards):
    for card in cards:
        validateCard(card)


def validateCard(card: Card):
    if card.suit == "R":
        if not (0 < card.value <= 4):
            raise werkzeug.exceptions.BadRequest("Bad card value")
    elif card.suit in "BYMG":
        if not (0 < card.value <= 9):
            raise werkzeug.exceptions.BadRequest("Bad card value")
    else:
        raise werkzeug.exceptions.BadRequest("Bad card suit")


def validateTrickIndex(trickIndex):
    if not (0 <= trickIndex < len(STATE.tricks)):
        raise werkzeug.exceptions.BadRequest("Bad trick index")


def validateTurnIndex(trickIndex, turnIndex):
    if not (0 <= turnIndex < len(STATE.tricks[trickIndex].turns)):
        raise werkzeug.exceptions.BadRequest("Bad turn index")


def validateObjectiveType(objtype):
    if objtype not in {"absolute", "relative", "anytime", "last"}:
        raise werkzeug.exceptions.BadRequest("Bad turn index")


def validateObjectiveOrder(objtype, order):
    if objtype in {"absolute", "relative"}:
        currMax = max((task.order for task in STATE.objectives.values()
                       if task.type == objtype), default=0)
        if order != currMax + 1:
            raise werkzeug.exceptions.BadRequest("Bad objective order")
    else:
        if order != 0:
            raise werkzeug.exceptions.BadRequest("Bad objective order")


def validateTaskId(taskId):
    if taskId not in STATE.objectives:
        raise werkzeug.exceptions.BadRequest("Bad task id")


def parsePlayerNum(playerNum):
    playerNum = int(playerNum)
    validatePlayerNum(playerNum)
    return playerNum


def parseCard(cardJson) -> Card:
    card = Card(cardJson["suit"], cardJson["value"])
    validateCard(card)
    return card


def parseCards(cardsJson) -> list[Card]:
    cards = [parseCard(c) for c in cardsJson]
    validateCards(cards)
    return cards


if __name__ == "__main__":
    socketio.run(app)
