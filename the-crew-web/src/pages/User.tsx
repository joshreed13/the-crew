import React from "react";
import { apiCall } from "../api";

export default function UserPage({ selectedPlayer, setSelectedPlayer }: { selectedPlayer: number | null, setSelectedPlayer: (value: number) => void }) {
    const reqSetName = (event: React.FormEvent) => {
        event.preventDefault();
        const nameInput = (event.target as HTMLFormElement).elements.namedItem("namefield") as HTMLInputElement;
        apiCall(`/api/player/${selectedPlayer}/name`, { name: nameInput.value });
    };

    return (
        <>
            <p>{selectedPlayer || "?"}</p>
            <div>
                <button onClick={() => setSelectedPlayer(0)}>Select 0</button>
                <button onClick={() => setSelectedPlayer(1)}>Select 1</button>
                <button onClick={() => setSelectedPlayer(2)}>Select 2</button>
                <button onClick={() => setSelectedPlayer(3)}>Select 3</button>
            </div>
            <form onSubmit={reqSetName} >
                <input type="text" name="namefield" />
                <input type="submit" value="Set Name" />
            </form>
        </>
    );
}
