import { Card, HandPageData } from "../model";
import { CardView } from "../Common";
import { MultiCardPicker } from "../CardPicker";
import { apiCall } from "../api";

export default function HandPage({ data, selectedPlayer }: { data: HandPageData, selectedPlayer: number | null }) {
    return (
        <>
            {data.heldCards.map((hand: Card[], i) => (
                <>
                    <div className="bordered">
                        {hand.map((card) => (<CardView card={card} />))}
                    </div>
                    <MultiCardPicker callback={(cards: Card[]) => {
                        apiCall(`/api/player/${i}/hand`, { cards: cards });
                    }} />
                </>
            ))}
        </>
    );
}
