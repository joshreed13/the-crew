import { Card, Player, Task, Trick, Turn } from "./model";
import "./Common.css";
import { CardPicker } from "./CardPicker";
import { apiCall } from "./api";

export function CardView({ card }: { card: Card }) {
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
    return <div>Task: {task.id} ({task.type} {task.order} {task.card?.suit}, {task.card?.value}) assigned to <PlayerName player={task.player} /></div>
}

export function PlayerName({ player }: { player: Player | undefined }) {
    if (player == undefined) {
        return <span>?</span>
    }
    else {
        return (
            <span>
                {player.name}
                {player.isCommander ? "🧑‍🚀" : ""}
            </span>
        );
    }
}

export function TrickView({ data, trickNum }: { data: Trick, trickNum: number }) {
    return (
        <div className="bordered trickContainer">
            {data.turns.map((turn, i) => (<TurnView data={turn} trickNum={trickNum} turnNum={i} />))}
        </div>
    );
}

function TurnView({ data, trickNum, turnNum }: { data: Turn, trickNum: number, turnNum: number }) {
    return (
        <div className="bordered turnContainer">
            <PlayerName player={data.player} />
            <p>{data.isLeader ? "[Leader] " : ""} {data.isNextToPlay ? "[Next] " : ""} {data.isWinner ? "[Winner] " : ""}</p>
            {data.card ? <CardView card={data.card} /> : <></>}
            <CardPicker callback={(card: Card) => {
                apiCall(`/api/trick/${trickNum}/${turnNum}/card`, { card: card });
            }} />
        </div>
    );
}
