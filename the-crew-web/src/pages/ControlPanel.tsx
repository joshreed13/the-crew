import { Button, Col, Row, Stack } from "react-bootstrap";
import { ControlPanelData, PlayerData, TricksPageData } from "../model";
import { CardView, PlayerName, TaskTokenView, TrickView } from "../Common";
import { apiCall } from "../api";

import "./ControlPanel.css"
import TricksPage from "./Tricks";

export default function ControlPanel({ data, tricksData, selectedPlayer }: { data: ControlPanelData, tricksData: TricksPageData, selectedPlayer: number | undefined }) {
    const resetGame = () => {
        if (window.confirm("Are you sure you want to reset the game?")) {
            apiCall(`/api/reset`, {}, "POST");
        }
    };

    return (
        <>
            <Row>
                <Button onClick={resetGame}>Reset Game</Button>
            </Row>
            <Row>
                {data.players.map((playerData) => (<PlayerView data={playerData} />))}
            </Row>
            <TricksPage data={tricksData} selectedPlayer={selectedPlayer} />
        </>
    );
}

function PlayerView({ data }: { data: PlayerData }) {
    return (
        <Row>
            <Col>
                <PlayerName player={data.player} />
            </Col>
            <Col>
                <Stack direction="horizontal" gap={1}>
                    {data.hand.map((card) => (
                        <CardView card={card} />
                    ))}
                </Stack>
            </Col>
            <Col>
                <Stack direction="horizontal" gap={1}>
                    {data.tasks.map((task) => (
                        <Stack direction="horizontal">
                            <div className="position-relative">
                                {task.card && <CardView card={task.card} />}
                                <span className="position-absolute top-50 start-100 small-token">
                                    <TaskTokenView taskType={task.type} order={task.order} />
                                </span>
                            </div>
                        </Stack>
                    ))}
                </Stack>
            </Col>
        </Row>
    );
}
