pub type CardSet = u64;
pub type CardIndex = u8;

pub mod suit {
    use super::CardSet;

    pub const BLUE: CardSet = 0b111111111;
    pub const YELLOW: CardSet = 0b111111111 << 9;
    pub const MAGENTA: CardSet = 0b111111111 << 18;
    pub const GREEN: CardSet = 0b111111111 << 27;
    pub const ROCKETS: CardSet = 0b1111 << 36;
}

pub mod cards {
    use super::CardIndex;

    pub const B1: CardIndex = 0;
    pub const B2: CardIndex = 1;
    pub const B3: CardIndex = 2;
    pub const B4: CardIndex = 3;
    pub const B5: CardIndex = 4;
    pub const B6: CardIndex = 5;
    pub const B7: CardIndex = 6;
    pub const B8: CardIndex = 7;
    pub const B9: CardIndex = 8;

    pub const Y1: CardIndex = 9;
    pub const Y2: CardIndex = 10;
    pub const Y3: CardIndex = 11;
    pub const Y4: CardIndex = 12;
    pub const Y5: CardIndex = 13;
    pub const Y6: CardIndex = 14;
    pub const Y7: CardIndex = 15;
    pub const Y8: CardIndex = 16;
    pub const Y9: CardIndex = 17;

    pub const M1: CardIndex = 18;
    pub const M2: CardIndex = 19;
    pub const M3: CardIndex = 20;
    pub const M4: CardIndex = 21;
    pub const M5: CardIndex = 22;
    pub const M6: CardIndex = 23;
    pub const M7: CardIndex = 24;
    pub const M8: CardIndex = 25;
    pub const M9: CardIndex = 26;

    pub const G1: CardIndex = 27;
    pub const G2: CardIndex = 28;
    pub const G3: CardIndex = 29;
    pub const G4: CardIndex = 30;
    pub const G5: CardIndex = 31;
    pub const G6: CardIndex = 32;
    pub const G7: CardIndex = 33;
    pub const G8: CardIndex = 34;
    pub const G9: CardIndex = 35;

    pub const R1: CardIndex = 36;
    pub const R2: CardIndex = 37;
    pub const R3: CardIndex = 38;
    pub const R4: CardIndex = 39;
}

pub fn card_at_position(index: CardIndex) -> CardSet {
    1 << index
}

pub fn card_list_to_set(cards: &[CardIndex]) -> CardSet {
    cards.iter().fold(0, |a, c| a | card_at_position(*c))
}

pub fn get_suit(card: CardSet) -> CardSet {
    if card & suit::BLUE != 0 {
        suit::BLUE
    } else if card & suit::YELLOW != 0 {
        suit::YELLOW
    } else if card & suit::MAGENTA != 0 {
        suit::MAGENTA
    } else if card & suit::GREEN != 0 {
        suit::GREEN
    } else if card & suit::ROCKETS != 0 {
        suit::ROCKETS
    } else {
        0
    }
}
