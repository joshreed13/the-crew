export type AppState = {
    handPage: HandPageState;
    objectivePage: ObjectivePageState;
    tricksPage: TricksPageState;
    controlPanel: ControlPanelState;
};

export type HandPageState = {
    heldCards: Card[];
};

export type ObjectivePageState = {
    tasks: Task[];
};

export type TricksPageState = {
    tricks: Trick[];
};

export type ControlPanelState = {
    players: PlayerState[];
    tricks: Trick[];
}

export type PlayerState = {
    player: Player;
    hand: Card[];
    tasks: Task[];
};

export type Task = {
    id: string;
    type: string;
    order: number;
    card?: Card;
    player?: Player;
};

export type Trick = {
    turns: Turn[];
};

export type Turn = {
    player: Player;
    card?: Card;
    isLeader: boolean;
    isWinner: boolean;
    isNextToPlay: boolean;
};

export type Player = {
    name: string;
    isCommander: boolean;
};

export type Card = {
    suit: string;
    value: number;
};
