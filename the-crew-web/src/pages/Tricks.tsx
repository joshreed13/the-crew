import { TricksPageState } from "../model";
import { TrickView } from "../Common";

export default function TricksPage({ state }: { state: TricksPageState }) {
    return (
        <div>
            {state.tricks.map((trickState) => (<TrickView state={trickState} />))}
        </div>
    );
}
