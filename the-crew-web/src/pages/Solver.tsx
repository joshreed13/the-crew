import { Col, ListGroup, Row } from "react-bootstrap";
import { Solve, SolverPageData } from "../model";

export default function SolverPage({ data }: { data: SolverPageData }) {
    return (
        <Row>
            <Col>
                <ListGroup>
                    {data.solves.map((solve: Solve) => (
                        <ListGroup.Item key={solve.id}>
                            <span>#{solve.id} {result(solve)} <i>{solve.duration}ms</i></span>
                        </ListGroup.Item>
                    ))}
                </ListGroup>
            </Col>
        </Row>
    );
}

function result(solve: Solve): string {
    if (solve.success && solve.result) {
        return "Winnable";
    }
    else if (solve.success && !solve.result) {
        return "Not winnable";
    }
    else {
        return "Computation failed";
    }
}
