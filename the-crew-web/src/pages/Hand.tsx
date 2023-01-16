import { Card, HandPageData } from "../model";
import { CardView } from "../Common";

export default function HandPage({ data, selectedPlayer }: { data: HandPageData, selectedPlayer: number | null }) {
    return (
        <>
            {data.heldCards.map((hand: Card[]) => (
                <div className="bordered">
                    {hand.map((card) => (<CardView card={card} />))}
                </div>
            ))}
        </>
    );
}
