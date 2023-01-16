import { useId } from "react";
import { CardView } from "./Common";
import { Card } from "./model";

const SUITS = ["B", "Y", "M", "G", "R"];

function numInSuit(suit: string) {
    return suit == "R" ? 4 : 9;
}

export function CardPicker({ callback }: { callback: (card: Card) => void }) {
    return (
        <table>
            {SUITS.map((suit) => (
                <tr>
                    <td>{suit}</td>
                    {Array.from(Array(numInSuit(suit)).keys()).map((i) => {
                        const card = { suit: suit, value: i + 1 };
                        return (
                            <td>
                                <button onClick={(e) => callback(card)}>
                                    <CardView card={card} />
                                </button>
                            </td>
                        )
                    })}
                </tr>
            ))}
        </table>
    );
}

export function MultiCardPicker({ callback }: { callback: (cards: Card[]) => void }) {
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
        <form onSubmit={submitPicker}>
            <table>
                <tbody>
                    {SUITS.map((suit) => (
                        <tr>
                            <td>{suit}</td>
                            {Array.from(Array(numInSuit(suit)).keys()).map((i) => {
                                const card = { suit: suit, value: i + 1 };
                                return (
                                    <td>
                                        <CardCheckbox card={card} />
                                    </td>
                                )
                            })}
                        </tr>
                    ))}
                </tbody>
            </table>
            <input type="submit" value="Submit" />
        </form>
    );
}

function CardCheckbox({ card }: { card: Card }) {
    const id = useId();
    return (
        <>
            <input type="checkbox" id={id} data-card={JSON.stringify(card)} />
            <label htmlFor={id}>
                <CardView card={card} />
            </label>
        </>
    );
}