import { TricksPageData } from "../model";
import { TrickView } from "../Common";
import { Col, Row } from "react-bootstrap";

export default function TricksPage({ data, selectedPlayer }: { data: TricksPageData, selectedPlayer: number | undefined }) {
    return (
        <Row>
            <Col>
                {data.tricks.map((trickData, i) => (<TrickView data={trickData} trickNum={i} heldCards={data.heldCards} selectedPlayer={selectedPlayer} />))}
            </Col>
        </Row>
    );
}
