import { Col, ListGroup, Row } from "react-bootstrap";
import { Card, HandPageData } from "../model";
import { CardView } from "../Common";
import { MultiCardPicker } from "../CardPicker";
import { apiCall } from "../api";

export default function HandPage({ data, selectedPlayer }: { data: HandPageData, selectedPlayer: number | undefined }) {
    if (selectedPlayer === undefined) {
        return <div>Please select a player on the User page to see your hand</div>;
    }
    else if (selectedPlayer < data.heldCards.length) {
        let hand = data.heldCards[selectedPlayer];
        return (
            <>
                <Row>
                    <Col>
                        <MultiCardPicker callback={(cards: Card[]) => {
                            apiCall(`/api/player/${selectedPlayer}/hand`, { cards: cards });
                        }} />
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <ListGroup horizontal>
                            {hand.map((card) => (
                                <ListGroup.Item>
                                    <CardView card={card} />
                                </ListGroup.Item>
                            ))}
                        </ListGroup>
                    </Col>
                </Row>
            </>
        );
    }
    else {
        return <div>Loading...</div>;
    }
}