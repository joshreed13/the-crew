import logging

from flask import Flask, request, Response, jsonify
import werkzeug.exceptions
from flask_socketio import SocketIO, emit

from round import Round, Card
from solver import Solver

logging.basicConfig(filename="audit.log",
                    format="%(asctime)s %(name)s %(levelname)s %(message)s",
                    datefmt="%Y-%m-%dT%H:%M:%S",
                    level=logging.INFO)

app = Flask(__name__)
socketio = SocketIO(app)


STATE = Round()
SOLVER = Solver()


@app.before_request
def log_request_info():
    logging.getLogger("payloads").info(
        f'{request.remote_addr} {request.method} {request.url} {request.get_data()}')


@socketio.on('connect')
def handle_connect(auth):
    print("New client connected")
    emit("appstate", STATE.toJson())


def publishUpdate():
    json = STATE.toJson()
    socketio.emit("appstate", json)
    logging.getLogger("state").info(json)


@app.route("/api/appstate", methods=['GET'])
def get_state():
    return jsonify(STATE.toJson())


@ app.route("/api/reset", methods=['POST'])
def resetGame():
    STATE.reset()
    publishUpdate()
    return Response("Success")


@ app.route("/api/player/<int:playerNum>/name", methods=['PUT'])
def setPlayerName(playerNum):
    json = validateRequestIsJson(request)
    playerNum = parsePlayerNum(playerNum)
    name = json["name"]

    STATE.setPlayerName(playerNum, name)
    publishUpdate()
    return Response("Success")


@ app.route("/api/player/<int:playerNum>/hand", methods=['PUT'])
def setPlayerHand(playerNum):
    json = validateRequestIsJson(request)
    playerNum = parsePlayerNum(playerNum)
    cards = parseCards(json["cards"])

    STATE.setPlayerHand(playerNum, cards)
    publishUpdate()
    return Response("Success")


@ app.route("/api/objective/add", methods=['POST'])
def addObjective():
    json = validateRequestIsJson(request)
    objtype = json["type"]
    validateObjectiveType(objtype)
    order = int(json["order"])
    validateObjectiveOrder(objtype, order)
    card = json.get("card")
    if card is not None:
        card = parseCard(card)
    playerNum = json.get("playerNum")
    if playerNum is not None:
        playerNum = parsePlayerNum(playerNum)

    STATE.addObjective(objtype, order, card, playerNum)
    publishUpdate()
    return Response("Success")


@ app.route("/api/objective/<int:id>", methods=['DELETE'])
def removeObjective(id):
    validateTaskId(id)

    STATE.removeObjective(id)
    publishUpdate()
    return Response("Success")


@ app.route("/api/objective/<int:id>/card", methods=['PUT'])
def setObjectiveCard(id):
    json = validateRequestIsJson(request)
    validateTaskId(id)
    card = parseCard(json["card"])

    STATE.setObjectiveCard(id, card)
    publishUpdate()
    return Response("Success")


@ app.route("/api/objective/<int:id>/player", methods=['PUT'])
def setObjectivePlayer(id):
    json = validateRequestIsJson(request)
    validateTaskId(id)
    playerNum = parsePlayerNum(json["playerNum"])

    STATE.setObjectivePlayer(id, playerNum)
    publishUpdate()
    return Response("Success")


@ app.route("/api/trick/<int:trickIndex>/<int:turnIndex>/card", methods=['PUT'])
def setTrickTurnCard(trickIndex, turnIndex):
    json = validateRequestIsJson(request)
    validateTrickIndex(trickIndex)
    validateTurnIndex(trickIndex, turnIndex)
    card = parseCard(json["card"])

    STATE.setTrickTurnCard(trickIndex, turnIndex, card)
    publishUpdate()
    return Response("Success")


@ app.route("/api/solve", methods=['POST'])
def solve():
    def callback(id, result):
        STATE.addSolverResult(id, result)
        publishUpdate()
    SOLVER.solve(STATE, callback)
    return Response("Success")


def validateRequestIsJson(request):
    json = request.get_json()
    if json is None:
        raise werkzeug.exceptions.UnsupportedMediaType("Expected JSON")
    return json


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
