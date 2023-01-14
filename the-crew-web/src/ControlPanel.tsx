import { CardView } from "./Card";
import { Card } from "./model";

export default function ControlPanel({ cards }: { cards: Card[] }) {
    return (
        <>
            {cards.map((card) => (<CardView card={card} />))}
        </>
    );
}
