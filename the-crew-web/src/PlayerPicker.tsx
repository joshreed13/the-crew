import { PlayerName } from "./Common";
import { Player } from "./model";

export function PlayerPicker({ players, selectedPlayerNum, callback }: { players: Player[], selectedPlayerNum: number | undefined, callback: (playerNum: number) => void }) {
    const submitPicker = (event: React.FormEvent) => {
        const select = event.target as HTMLFormElement;
        callback(select.value);
    };

    return (
        <select value={selectedPlayerNum} onChange={submitPicker}>
            <option></option>
            {players.map((player, i) => (
                <option value={i}>
                    <PlayerName player={player} />
                </option>
            ))}
        </select>
    );
}
