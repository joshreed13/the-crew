import { Button, ButtonGroup, ButtonToolbar, Card as BootstrapCard, Col, Row } from 'react-bootstrap';
import { Card, ObjectivePageData, Player, Task } from "../model";
import { apiCall } from "../api";
import { CardView, getTaskString, TaskTokenView } from "../Common";
import { CardPicker } from "../CardPicker";
import { PlayerPicker } from "../PlayerPicker";

export default function ObjectivesPage({ data }: { data: ObjectivePageData }) {
    const addObj = (objType: string, order: number) => {
        apiCall(`/api/objective/add`, { "type": objType, "order": order }, "POST");
    };

    return (
        <>
            <Row>
                <Col>
                    <ButtonToolbar>
                        <ButtonGroup>
                            <Button onClick={(e) => addObj("anytime", 0)}>Add</Button>
                        </ButtonGroup>
                        <ButtonGroup>
                            <Button onClick={(e) => addObj("absolute", data.nextAbsolute)}>Add {getTaskString("absolute", data.nextAbsolute)}</Button>
                        </ButtonGroup>
                        <ButtonGroup>
                            <Button onClick={(e) => addObj("relative", data.nextRelative)}>Add {getTaskString("relative", data.nextRelative)}</Button>
                        </ButtonGroup>
                        <ButtonGroup>
                            <Button onClick={(e) => addObj("last", 0)} disabled={data.haveLast}>Add {getTaskString("last", 0)}</Button>
                        </ButtonGroup>
                    </ButtonToolbar>
                </Col>
            </Row>
            <Row>
                <Col>
                    {data.tasks.map((task) => (
                        <TaskView task={task} players={data.players} />
                    ))}
                </Col>
            </Row>
        </>
    );
}

function TaskView({ task, players }: { task: Task, players: Player[] }) {
    const rmObj = (taskId: string) => {
        if (window.confirm("Are you sure you want to remove this objective?")) {
            apiCall(`/api/objective/${taskId}`, {}, "DELETE");
        }
    }
    const setCard = (taskId: string, card: Card) => {
        apiCall(`/api/objective/${taskId}/card`, { card: card });
    }
    const setPlayer = (taskId: string, playerNum: number) => {
        apiCall(`/api/objective/${taskId}/player`, { playerNum: playerNum });
    }

    return (
        <BootstrapCard key={task.id}>
            <BootstrapCard.Header>
                <Button className="float-end" variant='danger' onClick={(e) => rmObj(task.id)}>X</Button>
            </BootstrapCard.Header>
            <BootstrapCard.Body>
                <Row>
                    <Col>
                        <TaskTokenView taskType={task.type} order={task.order} />
                    </Col>
                    <Col>
                        {task.card && <CardView card={task.card} />}
                        <CardPicker cards={undefined} callback={(card: Card) => { setCard(task.id, card) }} />
                    </Col>
                    <Col>
                        <PlayerPicker players={players} selectedPlayerNum={task.player?.num} callback={(playerNum) => (setPlayer(task.id, playerNum))} />
                    </Col>
                </Row>
            </BootstrapCard.Body>
        </BootstrapCard>
    );
}
