import { CardView } from "./Card";
import { Card, Hand, RoundState, Task, User } from "./model";

export default function ControlPanel({ round }: { round: RoundState }) {
    return (
        <>
            <HandsView hands={round.hands} />
        </>
    );
}

function HandsView({ hands }: { hands: Hand[] }) {
    return (
        <>
            {hands.map((hand) => (<UserHandView user={hand.user} cards={hand.cards} />))}
        </>
    );
}

function UserHandView({ user, cards, tasks }: { user: User, cards: Card[], tasks: Task[] }) {
    return (
        <div>
            <p>{user.name}</p>
            {cards.map((card) => (<CardView card={card} />))}
        </div>
    );
}