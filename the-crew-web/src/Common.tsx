import { Card, Player, Task, Trick, Turn } from "./model";
import "./Common.css";

export function CardView({ card }: { card: Card }) {
    let suitName = "?"
    switch (card.suit) {
        case "B": return <div className="cardContainer blueCard">{card.value}</div>;
        case "Y": return <div className="cardContainer yellowCard">{card.value}</div>
        case "M": return <div className="cardContainer magentaCard">{card.value}</div>
        case "G": return <div className="cardContainer greenCard">{card.value}</div>
        case "R": return <div className="cardContainer rocketCard">{card.value}</div>
        default: return <div className="cardContainer">? {card.value}</div>
    }
}

export function TaskView({ task }: { task: Task }) {
    return <div>Task: {task.id} ({task.type} {task.order} {task.card?.suit}, {task.card?.value}) assigned to {task.player?.name}</div>
}

export function PlayerName({ player }: { player: Player }) {
    return (
        <p>
            {player.name}
            {player.isCommander ? "🧑‍🚀" : ""}
        </p>
    );
}

export function TrickView({ state }: { state: Trick }) {
    return (
        <div className="bordered">
            {state.turns.map((turn) => (<TurnView state={turn} />))}
        </div>
    );
}

function TurnView({ state }: { state: Turn }) {
    return (
        <div className="bordered">
            <PlayerName player={state.player} />
            <p>{state.isLeader ? "[Leader] " : ""} {state.isNextToPlay ? "[Next] " : ""} {state.isWinner ? "[Winner] " : ""}</p>
            {state.card ? <CardView card={state.card} /> : <></>}
        </div>
    );
}
