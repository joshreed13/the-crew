import { ControlPanelData, PlayerData } from "../model";
import { CardView, PlayerName, TaskView, TrickView } from "../Common";

export default function ControlPanel({ data }: { data: ControlPanelData }) {
    return (
        <div>
            <div>
                {data.players.map((playerData) => (<PlayerView data={playerData} />))}
            </div>
            <div>
                {data.tricks.map((trickData) => (<TrickView data={trickData} />))}
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
