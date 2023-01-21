import { Badge, Card as BootstrapCard, Col, Container, Row, Stack } from 'react-bootstrap';
import { Card, Player, Trick, Turn } from "./model";
import { CardPicker } from "./CardPicker";
import { apiCall } from "./api";

import "./Common.css";

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

export function getPlayerName(player: Player | undefined): string {
    if (player === undefined) {
        return "?"
    }
    else {
        return `${player.name}${player.isCommander ? "🧑‍🚀" : ""}`;
    }
}

export function TrickView({ data, trickNum, heldCards, selectedPlayer }: { data: Trick, trickNum: number, heldCards: Card[][], selectedPlayer: number | undefined }) {
    return (
        <BootstrapCard>
            <Container>
                <Row>
                    {data.turns.map((turn, i) => {
                        const possibleCards = getPossibleCards(i, heldCards, selectedPlayer);
                        return (
                            <Col>
                                <TurnView data={turn} trickNum={trickNum} turnNum={i} possibleCards={possibleCards} />
                            </Col>
                        )
                    })}
                </Row>
            </Container>
        </BootstrapCard>
    );
}

function TurnView({ data, trickNum, turnNum, possibleCards }: { data: Turn, trickNum: number, turnNum: number, possibleCards: Card[] }) {
    return (
        <div>
            <Container>
                <Row>
                    <Col>
                        <span>{getPlayerName(data.player)}</span>
                        {data.isLeader && <Badge bg="secondary">Leader</Badge>}
                        {data.isNextToPlay && <Badge bg="success">Next</Badge>}
                        {data.isWinner && <Badge bg="warning">Winner</Badge>}
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <Stack>
                            {<CardPicker cards={possibleCards} callback={(card: Card) => {
                                apiCall(`/api/trick/${trickNum}/${turnNum}/card`, { card: card });
                            }} />}
                            {data.card && <CardView card={data.card} />}
                        </Stack>
                    </Col>
                </Row>
            </Container>
        </div >
    );
}

function getPossibleCards(forPlayer: number, heldCards: Card[][], selectedPlayer: number | undefined): Card[] {
    if (forPlayer === selectedPlayer) {
        if (forPlayer < heldCards.length) {
            return heldCards[forPlayer];
        }
        else {
            return [];
        }
    }
    else {
        let possibleCards: Card[] = [];
        for (let i = 0; i < heldCards.length; i++) {
            if (i != selectedPlayer) {
                possibleCards = possibleCards.concat(heldCards[i]);
            }
        }
        return possibleCards;
    }
}