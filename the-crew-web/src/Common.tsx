import { Badge, Card as BootstrapCard, Col, Container, ListGroup, Row } from 'react-bootstrap';
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

export function TaskTokenView({ taskType, order }: { taskType: string, order: number }) {
    return (
        <div className="cardContainer tokenCard">
            {getTaskString(taskType, order)}
        </div>
    );
}

export function getTaskString(taskType: string, taskOrder: number) {
    switch (taskType) {
        case "absolute": return taskOrder;
        case "relative": return "<".repeat(taskOrder);
        case "anytime": return "";
        case "last": return "Ω";
        default: return "";
    }
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
        <BootstrapCard>
            <Container>
                <Row>
                    {data.turns.map((turn, i) => (
                        <Col>
                            <TurnView data={turn} trickNum={trickNum} turnNum={i} />
                        </Col>
                    ))}
                </Row>
            </Container>
        </BootstrapCard>
    );
}

function TurnView({ data, trickNum, turnNum }: { data: Turn, trickNum: number, turnNum: number }) {
    return (
        <div>
            <Container>
                <Row>
                    <Col>
                        <PlayerName player={data.player} />
                        {data.isLeader && <Badge bg="secondary">Leader</Badge>}
                        {data.isNextToPlay && <Badge bg="success">Next</Badge>}
                        {data.isWinner && <Badge bg="warning">Winner</Badge>}
                    </Col>
                </Row>
                <Row>
                    <Col>
                        {<CardPicker callback={(card: Card) => {
                            apiCall(`/api/trick/${trickNum}/${turnNum}/card`, { card: card });
                        }} />}
                        {data.card && <CardView card={data.card} />}
                    </Col>
                </Row>
            </Container>
        </div >
    );
}
