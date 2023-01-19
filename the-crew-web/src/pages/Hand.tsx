import { Card, HandPageData } from "../model";
import { CardView } from "../Common";
import { MultiCardPicker } from "../CardPicker";
import { apiCall } from "../api";

export default function HandPage({ data, selectedPlayer }: { data: HandPageData, selectedPlayer: number | undefined }) {
    if (selectedPlayer === undefined) {
        return <div>Please select a player on the User page to see your hand</div>;
    }
    else if (selectedPlayer < data.heldCards.length) {
        let hand = data.heldCards[selectedPlayer];
        return (
            <>
                <div className="bordered">
                    {hand.map((card) => (<CardView card={card} />))}
                </div>
                <MultiCardPicker callback={(cards: Card[]) => {
                    apiCall(`/api/player/${selectedPlayer}/hand`, { cards: cards });
                }} />
            </>
        );
    }
    else {
        return <div>Loading...</div>;
    }
}