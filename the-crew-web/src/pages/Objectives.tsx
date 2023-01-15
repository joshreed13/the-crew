import { ObjectivePageState } from "../model";
import { TaskView } from "../Common";

export default function ObjectivesPage({ state }: { state: ObjectivePageState }) {
    return (
        <ol>
            {state.tasks.map((task) => (
                <li>
                    <TaskView task={task} />
                </li>
            ))}
        </ol>
    );
}
