import { Card, ObjectivePageData, Player, Task } from "../model";
import { apiCall } from "../api";
import { CardView, PlayerName } from "../Common";
import { CardPicker } from "../CardPicker";
import { PlayerPicker } from "../PlayerPicker";

export default function ObjectivesPage({ data }: { data: ObjectivePageData }) {
    const addObj = (objType: string, order: number) => {
        apiCall(`/api/objective/add`, { "type": objType, "order": order }, "POST");
    };

    return (
        <>
            <div>
                <button onClick={(e) => addObj("anytime", 0)}>Add</button>
                <button onClick={(e) => addObj("absolute", data.nextAbsolute)}>Add {getTaskString("absolute", data.nextAbsolute)}</button>
                <button onClick={(e) => addObj("relative", data.nextRelative)}>Add {getTaskString("absolute", data.nextRelative)}</button>
                <button onClick={(e) => addObj("last", 0)} disabled={data.haveLast}>Add {getTaskString("last", 0)}</button>
            </div>
            <ol>
                {data.tasks.map((task) => (
                    <li key={task.id}>
                        <TaskView task={task} players={data.players} />
                    </li>
                ))}
            </ol>
        </>
    );
}

function TaskView({ task, players }: { task: Task, players: Player[] }) {
    const rmObj = (taskId: string) => {
        apiCall(`/api/objective/${taskId}`, {}, "DELETE");
    }
    const setCard = (taskId: string, card: Card) => {
        apiCall(`/api/objective/${taskId}/card`, { card: card });
    }
    const setPlayer = (taskId: string, playerNum: number) => {
        apiCall(`/api/objective/${taskId}/player`, { playerNum: playerNum });
    }

    return (
        <div className="bordered">
            {getTaskString(task.type, task.order)}
            {task.card && <CardView card={task.card} />}
            <CardPicker callback={(card: Card) => { setCard(task.id, card) }} />
            <PlayerPicker players={players} selectedPlayerNum={task.player?.num} callback={(playerNum) => (setPlayer(task.id, playerNum))} />
            <button onClick={(e) => rmObj(task.id)}>X</button>
        </div>
    );
}

function getTaskString(taskType: string, taskOrder: number) {
    switch (taskType) {
        case "absolute": return `#${taskOrder}`;
        case "relative": return "<".repeat(taskOrder);
        case "anytime": return "";
        case "last": return "Ω";
        default: return "";
    }
}