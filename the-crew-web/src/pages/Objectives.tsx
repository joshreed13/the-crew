import { ObjectivePageData } from "../model";
import { TaskView } from "../Common";

export default function ObjectivesPage({ data }: { data: ObjectivePageData }) {
    return (
        <ol>
            {data.tasks.map((task) => (
                <li>
                    <TaskView task={task} />
                </li>
            ))}
        </ol>
    );
}
