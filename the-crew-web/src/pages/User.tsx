import React from "react";
import { Button, Col, Form, InputGroup, Row } from "react-bootstrap";
import { apiCall } from "../api";
import { Player } from "../model";
import { PlayerPicker } from "../PlayerPicker";

export default function UserPage({ players, selectedPlayer, setSelectedPlayer }: { players: Player[], selectedPlayer: number | undefined, setSelectedPlayer: (value: number) => void }) {
    const reqSetName = (event: React.FormEvent) => {
        event.preventDefault();
        const nameInput = (event.target as HTMLFormElement).elements.namedItem("namefield") as HTMLInputElement;
        apiCall(`/api/player/${selectedPlayer}/name`, { name: nameInput.value });
    };

    return (
        <>
            <Row>
                <Col>
                    <PlayerPicker players={players} selectedPlayerNum={selectedPlayer} callback={(num) => setSelectedPlayer(num)} />
                </Col>
            </Row>
            <Row>
                <Form onSubmit={reqSetName} >
                    <InputGroup>
                        <Form.Control type="text" name="namefield" placeholder="Set player name" />
                        <Button variant="primary" as="input" type="submit" value="Set Name" />
                    </InputGroup>
                </Form>
            </Row>
            <Row>
                <Col>
                    [Selected Player Code {(selectedPlayer === undefined) ? "?" : selectedPlayer}]
                </Col>
            </Row>
        </>
    );
}
