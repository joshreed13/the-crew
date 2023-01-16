import { HandPageData } from "../model";
import { CardView } from "../Common";

export default function HandPage({ data }: { data: HandPageData }) {
    return (
        <div className="bordered">
            {data.heldCards.map((card) => (<CardView card={card} />))}
        </div>
    );
}
