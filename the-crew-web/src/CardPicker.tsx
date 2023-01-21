import { useId, useState } from "react";
import { Button, Form, Modal, Stack } from "react-bootstrap";
import { CardView } from "./Common";
import { Card } from "./model";

const SUITS = ["B", "Y", "M", "G", "R"];

function numInSuit(suit: string) {
    return suit == "R" ? 4 : 9;
}

export function CardPicker({ callback }: { callback: (card: Card) => void }) {
    const [show, setShow] = useState(false);

    const closeModal = () => setShow(false);
    const showModal = () => setShow(true);

    const submitPicker = (card: Card) => {
        callback(card);
        closeModal();
    };

    return (
        <>
            <Button variant="primary" onClick={showModal}>
                Choose Card...
            </Button>
            <Modal show={show} onHide={closeModal}>
                <Modal.Header closeButton>
                    <Modal.Title>Choose Card</Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    <Stack direction="horizontal">
                        {SUITS.map((suit) => (
                            <Stack direction="vertical">
                                {Array.from(Array(numInSuit(suit)).keys()).map((i) => {
                                    const card = { suit: suit, value: i + 1 };
                                    return (
                                        <Button variant="outline-dark" onClick={(e) => submitPicker(card)}>
                                            <CardView card={card} />
                                        </Button>
                                    )
                                })}
                            </Stack>
                        ))}
                    </Stack>
                </Modal.Body>
            </Modal>
        </>
    );
}

export function MultiCardPicker({ callback }: { callback: (cards: Card[]) => void }) {
    const [show, setShow] = useState(false);

    const closeModal = () => setShow(false);
    const showModal = () => setShow(true);

    const submitPicker = (event: React.FormEvent) => {
        event.preventDefault();

        const elems = Array.from((event.target as HTMLFormElement).elements);
        const cards: Card[] = [];
        for (let i = 0; i < elems.length; i++) {
            const elem = elems[i] as HTMLInputElement
            if (elem.type === 'checkbox' && elem.checked) {
                cards.push(JSON.parse(elem.dataset.card as string));
            }
        }
        callback(cards);
    };

    return (
        <>
            <Button variant="primary" onClick={showModal}>
                Choose Cards...
            </Button>
            <Modal show={show} onHide={closeModal}>
                <form onSubmit={submitPicker}>
                    <Modal.Header closeButton>
                        <Modal.Title>Choose Cards</Modal.Title>
                    </Modal.Header>
                    <Modal.Body>
                        <Stack direction="horizontal">
                            {SUITS.map((suit) => (
                                <Stack direction="vertical">
                                    {Array.from(Array(numInSuit(suit)).keys()).map((i) => {
                                        const card = { suit: suit, value: i + 1 };
                                        return (
                                            <CardCheckbox card={card} />
                                        )
                                    })}
                                </Stack>
                            ))}
                        </Stack>
                    </Modal.Body>
                    <Modal.Footer>
                        <Button variant="secondary" as="input" type="reset" onClick={closeModal} value="Cancel" />
                        <Button variant="primary" as="input" type="submit" onClick={closeModal} value="Save Changes" />
                    </Modal.Footer>
                </form>
            </Modal>
        </>
    );
}

function CardCheckbox({ card }: { card: Card }) {
    const id = useId();
    return (
        <>
            <Form.Check type="checkbox" id={id} data-card={JSON.stringify(card)} />
            <label htmlFor={id}>
                <CardView card={card} />
            </label>
        </>
    );
}