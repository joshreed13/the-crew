import React from "react";
import { apiCall } from "../api";
import { Player } from "../model";
import { PlayerPicker } from "../PlayerPicker";

export default function UserPage({ players, selectedPlayer, setSelectedPlayer }: { players: Player[], selectedPlayer: number | undefined, setSelectedPlayer: (value: number) => void }) {
    const reqSetName = (event: React.FormEvent) => {
        event.preventDefault();
        const nameInput = (event.target as HTMLFormElement).elements.namedItem("namefield") as HTMLInputElement;
        apiCall(`/api/player/${selectedPlayer}/name`, { name: nameInput.value });
    };

    return (
        <>
            <p>{selectedPlayer || "?"}</p>
            <div>
                <PlayerPicker players={players} selectedPlayerNum={selectedPlayer} callback={(num) => setSelectedPlayer(num)} />
            </div>
            <form onSubmit={reqSetName} >
                <input type="text" name="namefield" />
                <input type="submit" value="Set Name" />
            </form>
        </>
    );
}
