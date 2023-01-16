from flask import Flask, request, Response, jsonify
import werkzeug.exceptions
from flask_socketio import SocketIO, emit
from model import *

app = Flask(__name__)
# app.config['SECRET_KEY'] = "secret!"
socketio = SocketIO(app, async_mode='threading')
# logger=True, engineio_logger=True

TASKID = 1
STATE: RoundState = RoundState(
    players=[
        PlayerState("", []),
        PlayerState("", []),
        PlayerState("", []),
        PlayerState("", [])
    ],
    objectives={},
    tricks=[Trick([None, None, None, None])],
)


@socketio.on('connect')
def handle_connect(auth):
    print("Saw connection!")


@socketio.on('ping')
def handle_ping():
    print("Ping message!")
    emit("pong")


def publishUpdate():
    socketio.emit("appstate", buildStateJson())


@app.route("/api/test")
def index():
    #    return """<html><head><script src="https://cdnjs.cloudflare.com/ajax/libs/socket.io/4.0.1/socket.io.js" integrity="sha512-q/dWJ3kcmjBLU4Qc47E4A9kTB4m3wuTY7vkFJDTZKjTs8jhyGQnaUrxa0Ytd0ssMZhbNua9hE+E7Qv1j+DyZwA==" crossorigin="anonymous"></script>
    # <script type="text/javascript" charset="utf-8">
    # var socket = io({ transports: ["websocket"] });
    # </script></head><body>Hi</body></html>"""
    return """<html><head><script src="https://cdnjs.cloudflare.com/ajax/libs/socket.io/4.0.1/socket.io.js" integrity="sha512-q/dWJ3kcmjBLU4Qc47E4A9kTB4m3wuTY7vkFJDTZKjTs8jhyGQnaUrxa0Ytd0ssMZhbNua9hE+E7Qv1j+DyZwA==" crossorigin="anonymous"></script>
<script type="text/javascript" charset="utf-8">
    var socket = io({ transports: ["websocket"] });
    socket.on('connect', function() {
        //socket.emit('my event', {data: 'I\\'m connected!'});
        document.write("Connected!!!");
        console.log("Connected!!!");
    });
</script></head><body>Hi</body></html>"""


@app.route("/api/appstate", methods=['GET'])
def get_state():
    return jsonify(buildStateJson())


def buildStateJson():
    selectedPlayer = 0

    def toPlayer(player):
        return {
            "name": player.name,
            "isCommander": Card("R", 4) in player.hand
        }

    def toCard(card):
        return {
            "suit": card.suit,
            "value": card.value
        } if card is not None else None

    def toTask(id, task):
        return {
            "id": id,
            "type": task.type,
            "order": task.order,
            "card": toCard(task.card),
            "player": task.player,
        }

    tricks = [{
        "turns": [{
            "player": toPlayer(player),
            "card": toCard(card),
            "isLeader": False,
            "isWinner": False,
            "isNextToPlay": False,
        } for player, card in zip(STATE.players, trick.turns)]
    } for trick in STATE.tricks]

    return {
        "handPage": {
            "heldCards": [toCard(card) for card in STATE.players[selectedPlayer].hand]
        },
        "objectivePage": {
            "tasks": [toTask(id, task) for id, task in STATE.objectives.items()]
        },
        "tricksPage": {
            "tricks": tricks
        },
        "controlPanel": {
            "players": [{
                "player": toPlayer(player),
                "hand": [toCard(card) for card in player.hand],
                "tasks": [toTask(id, task) for id, task in STATE.objectives.items() if task.player == i],
            } for i, player in enumerate(STATE.players)],
            "tricks": tricks
        },
    }


@ app.route("/api/player/<int:playerNum>/name", methods=['PUT'])
def setPlayerName(playerNum):
    validateRequestIsJson(request)
    playerNum = parsePlayerNum(playerNum)

    name = request.get_json()["name"]
    STATE.players[playerNum].name = name

    return Response("Success")


@ app.route("/api/player/<int:playerNum>/hand", methods=['PUT'])
def setPlayerHand(playerNum):
    validateRequestIsJson(request)
    playerNum = parsePlayerNum(playerNum)

    cards = parseCards(request.get_json()["cards"])
    STATE.players[playerNum].hand = cards

    return Response("Success")


@ app.route("/api/objective/add", methods=['POST'])
def addObjective():
    global TASKID
    validateRequestIsJson(request)

    objtype = request.get_json()["type"]
    validateObjectiveType(objtype)
    order = int(request.get_json()["order"])
    validateObjectiveOrder(objtype, order)
    card = parseCard(request.get_json()["card"])
    playerNum = parsePlayerNum(request.get_json()["playerNum"])

    STATE.objectives[TASKID] = Task(objtype, order, card, playerNum)
    TASKID += 1

    return Response("Success")


@ app.route("/api/objective/<int:id>", methods=['DELETE'])
def removeObjective(id):
    validateTaskId(id)

    del STATE.objectives[id]

    return Response("Success")


@ app.route("/api/objective/<int:id>/card", methods=['PUT'])
def setObjectiveCard(id):
    validateRequestIsJson(request)
    validateTaskId(id)

    card = parseCard(request.get_json()["card"])
    STATE.objectives[id].card = card

    return Response("Success")


@ app.route("/api/objective/<int:id>/player", methods=['PUT'])
def setObjectivePlayer(id):
    validateRequestIsJson(request)
    validateTaskId(id)

    playerNum = parsePlayerNum(request.get_json()["playerNum"])
    STATE.objectives[id].player = playerNum

    return Response("Success")


@ app.route("/api/trick/<int:trickIndex>/<int:turnIndex>/card", methods=['PUT'])
def setTrickTurnCard(trickIndex, turnIndex):
    validateRequestIsJson(request)
    validateTrickIndex(trickIndex)
    validateTurnIndex(trickIndex, turnIndex)

    card = parseCard(request.get_json()["card"])
    STATE.tricks[trickIndex].turns[turnIndex] = card

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
    currMax = max(obj.order for id, obj in STATE.objectives.items()
                  if obj.type == objtype) if STATE.objectives else 0
    if not (0 <= order <= currMax):
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
