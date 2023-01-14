import { Card } from "./model";

export default function ControlPanel({ card }: { card: Card }) {
    return <CardComponent card={card} />;
}

function CardComponent({ card }: { card: Card }) {
    let suitName = "?"
    switch (card.suit) {
        case "B": suitName = "Blue"; break;
        case "Y": suitName = "Yellow"; break;
        case "M": suitName = "Magenta"; break;
        case "G": suitName = "Green"; break;
        case "R": suitName = "Rocket"; break;
    }
    return <div className="card">{suitName} {card.value}</div>
}