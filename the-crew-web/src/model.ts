export type AppState = {
    handPage: HandPageData;
    objectivePage: ObjectivePageData;
    tricksPage: TricksPageData;
    controlPanel: ControlPanelData;
    solverPage: SolverPageData;
};

export type HandPageData = {
    heldCards: Card[][];
};

export type ObjectivePageData = {
    tasks: Task[];
    nextAbsolute: number;
    nextRelative: number;
    haveLast: boolean;
    players: Player[];
};

export type TricksPageData = {
    tricks: Trick[];
    heldCards: Card[][];
};

export type ControlPanelData = {
    players: PlayerData[];
}

export type SolverPageData = {
    solves: Solve[];
}

export type PlayerData = {
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

export type Solve = {
    id: number;
    success: boolean;
    result: boolean;
    duration: number;
};

export type Player = {
    num: number;
    name: string;
    isCommander: boolean;
};

export type Card = {
    suit: string;
    value: number;
};
