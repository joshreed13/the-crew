import { ControlPanelData, PlayerData } from "../model";
import { CardView, PlayerName, TaskView, TrickView } from "../Common";
import { apiCall } from "../api";

export default function ControlPanel({ data }: { data: ControlPanelData }) {
    const resetGame = () => {
        if (window.confirm("Are you sure you want to reset the game?")) {
            apiCall(`/api/reset`, {}, "POST");
        }
    };

    return (
        <div>
            <button onClick={resetGame}>Reset Game</button>
            <div>
                {data.players.map((playerData) => (<PlayerView data={playerData} />))}
            </div>
            <div>
                {data.tricks.map((trickData, i) => (<TrickView data={trickData} trickNum={i} />))}
            </div>
        </div>
    );
}

function PlayerView({ data }: { data: PlayerData }) {
    return (
        <div className="bordered">
            <PlayerName player={data.player} />
            {data.hand.map((card) => (<CardView card={card} />))}
            {data.tasks.map((task) => (<TaskView task={task} />))}
        </div>
    );
}
