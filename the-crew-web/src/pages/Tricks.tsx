import { TricksPageData } from "../model";
import { TrickView } from "../Common";
import { Col, Row } from "react-bootstrap";

export default function TricksPage({ data }: { data: TricksPageData }) {
    return (
        <Row>
            <Col>
                {data.tricks.map((trickData, i) => (<TrickView data={trickData} trickNum={i} />))}
            </Col>
        </Row>
    );
}
