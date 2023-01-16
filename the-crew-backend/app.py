from flask import Flask, request, Response, jsonify
app = Flask(__name__)


@app.route("/api/hand/<playerNum>", methods=['GET'])
def set_hand(playerNum):
    request.get_json()
    # return jsonify(message=f"Hello Player {playerNum}")
    return Response("Success")
