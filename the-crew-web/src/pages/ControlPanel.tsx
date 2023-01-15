import { ControlPanelState, PlayerState } from "../model";
import { CardView, PlayerName, TaskView, TrickView } from "../Common";

export default function ControlPanel({ state }: { state: ControlPanelState }) {
    return (
        <div>
            <div>
                {state.players.map((playerState) => (<PlayerView state={playerState} />))}
            </div>
            <div>
                {state.tricks.map((trickState) => (<TrickView state={trickState} />))}
            </div>
        </div>
    );
}

function PlayerView({ state }: { state: PlayerState }) {
    return (
        <div className="bordered">
            <PlayerName player={state.player} />
            {state.hand.map((card) => (<CardView card={card} />))}
            {state.tasks.map((task) => (<TaskView task={task} />))}
        </div>
    );
}
