export type RoundState = {
    users: User[];
    hands: Hand[];
    taskObjectives: TaskObjective;
    tricks: Trick[];
};

type UserId = number;

export type User = {
    id: UserId;
    name?: string;
    left?: UserId;
    right?: UserId;
    isCommander: boolean;
};

export type Hand = {
    cards: Card[];
};

export type TaskObjective = {
    tasks: Task[];
};

export type Task = {
    id: number;
    type: string;
    index: number;
    card?: Card;
};

export type Trick = {
    leader: UserId;
    index: number;
    plays: Play[];
};

export type Play = {
    user: UserId;
    card?: Card;
    winner: boolean;
};

export type Card = {
    suit: string;
    value: number;
};
