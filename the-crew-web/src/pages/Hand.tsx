import { HandPageState } from "../model";
import { CardView } from "../Common";

export default function HandPage({ state }: { state: HandPageState }) {
    return (
        <div className="bordered">
            {state.heldCards.map((card) => (<CardView card={card} />))}
        </div>
    );
}
