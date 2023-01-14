import { Card } from "./model";
import "./Card.css";

export function CardView({ card }: { card: Card }) {
    let suitName = "?"
    switch (card.suit) {
        case "B": return <div className="cardContainer blueCard">{card.value}</div>;
        case "Y": return <div className="cardContainer yellowCard">{card.value}</div>
        case "M": return <div className="cardContainer magentaCard">{card.value}</div>
        case "G": return <div className="cardContainer greenCard">{card.value}</div>
        case "R": return <div className="cardContainer rocketCard">{card.value}</div>
        default: return <div className="cardContainer">? {card.value}</div>
    }
}