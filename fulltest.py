from datetime import datetime
import solver
from model import *
from cards import *

state = GameState([
    PlayerState("P1", hand=[B3, G1, M6, R4, B4, M3, Y2, Y6, B5, G9],
                tasks=[Task(Card(Suit.Blue, 8))]),
    PlayerState("P2", hand=[Y4, M5, G8, M4, G7, B1, R3, M7, Y7, B9], tasks=[]),
    PlayerState("P3", hand=[Y9, G5, Y5, M1, M9, Y3, B7, M2, M8, G3], tasks=[]),
    PlayerState("P4", hand=[R2, Y8, Y1, B6, B2, R1, G6, B8, G4, G2], tasks=[]),
])

start = datetime.now()
result = solver.solve(state)
if result is None:
    print("Unwinnable")
else:
    for play in result:
        print([card for card, remaining in play])

print((datetime.now() - start).total_seconds())
