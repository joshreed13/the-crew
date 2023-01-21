import { Button, Col, ListGroup, Row } from "react-bootstrap";
import { apiCall } from "../api";
import { Solve, SolverPageData } from "../model";

export default function SolverPage({ data }: { data: SolverPageData }) {
    const solveGame = () => {
        if (window.confirm("Are you sure you want to runt the solver?")) {
            apiCall(`/api/solve`, {}, "POST");
        }
    };

    return (
        <>
            <Row>
                <Col>
                    <Button onClick={solveGame}>Solve!</Button>
                </Col>
            </Row>
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
        </>
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
