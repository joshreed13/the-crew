use std::ops::{BitAnd, BitOr, Not};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Card {
    B1 = 0,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,

    Y1,
    Y2,
    Y3,
    Y4,
    Y5,
    Y6,
    Y7,
    Y8,
    Y9,

    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    M9,

    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    G9,

    R1,
    R2,
    R3,
    R4,
}

pub type RawCardSet = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CardSet(RawCardSet);

impl CardSet {
    pub const EMPTY: Self = CardSet(0);
    pub const EVERYTHING: Self = CardSet(RawCardSet::MAX);

    pub const fn from_card(card: Card) -> Self {
        CardSet(1 << (card as u8))
    }

    pub fn from_bit_index(value: u32) -> Self {
        CardSet(1 << value)
    }

    pub fn from_cards(cards: &[Card]) -> Self {
        cards
            .iter()
            .map(|c| CardSet::from_card(*c))
            .fold(CardSet::EMPTY, |a, c| a.add(c))
    }

    pub fn from_raw(raw: RawCardSet) -> Self {
        CardSet(raw)
    }

    pub fn add(&self, cards: Self) -> Self {
        CardSet(self.0 | cards.0)
    }

    pub fn contains(&self, card: Card) -> bool {
        self.overlaps_with(Self::from_card(card))
    }

    pub fn overlaps_with(&self, cards: CardSet) -> bool {
        *self & cards != Self::EMPTY
    }

    pub fn get_raw(&self) -> RawCardSet {
        self.0
    }

    pub fn get_suit(&self) -> Self {
        if self.0 & suit::BLUE.0 != 0 {
            suit::BLUE
        } else if self.0 & suit::YELLOW.0 != 0 {
            suit::YELLOW
        } else if self.0 & suit::MAGENTA.0 != 0 {
            suit::MAGENTA
        } else if self.0 & suit::GREEN.0 != 0 {
            suit::GREEN
        } else if self.0 & suit::ROCKETS.0 != 0 {
            suit::ROCKETS
        } else {
            CardSet::EMPTY
        }
    }

    pub fn highest_card(&self) -> Self {
        Self::from_bit_index(RawCardSet::BITS - self.0.leading_zeros() - 1)
    }

    pub fn is_covered_by(&self, cover: CardSet) -> bool {
        *self & !cover == Self::EMPTY
    }

    pub fn num_set(&self) -> u32 {
        self.0.count_ones()
    }

    const fn from_constant(cards: &[Card]) -> Self {
        match cards {
            [head, tail @ ..] => Self(Self::from_constant(tail).0 | Self::from_card(*head).0),
            _ => Self::EMPTY,
        }
    }
}

impl BitAnd for CardSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for CardSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for CardSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

pub mod suit {
    use super::Card::*;
    use super::CardSet;

    pub const BLUE: CardSet = CardSet::from_constant(&[B1, B2, B3, B4, B5, B6, B7, B8, B9]);
    pub const YELLOW: CardSet = CardSet::from_constant(&[Y1, Y2, Y3, Y4, Y5, Y6, Y7, Y8, Y9]);
    pub const MAGENTA: CardSet = CardSet::from_constant(&[M1, M2, M3, M4, M5, M6, M7, M8, M9]);
    pub const GREEN: CardSet = CardSet::from_constant(&[G1, G2, G3, G4, G5, G6, G7, G8, G9]);
    pub const ROCKETS: CardSet = CardSet::from_constant(&[R1, R2, R3, R4]);
}

#[cfg(test)]
mod tests {
    use super::Card::*;
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(CardSet::EMPTY.0, 0);
    }

    #[test]
    fn test_from_card() {
        assert_eq!(CardSet::from_card(B1).0, 0b1);
        assert_eq!(CardSet::from_card(B2).0, 0b10);
        assert_eq!(CardSet::from_card(B3).0, 0b100);
        assert_eq!(CardSet::from_card(B4).0, 0b1000);
    }

    #[test]
    fn test_from_cards() {
        assert_eq!(CardSet::from_cards(&[B1, B3, B4]).0, 0b1101);
    }

    #[test]
    fn test_from_bit_index() {
        assert_eq!(CardSet::from_bit_index(5).0, 0b100000);
    }

    #[test]
    fn test_from_raw() {
        assert_eq!(CardSet::from_raw(0b10110).0, 0b10110);
    }

    #[test]
    fn test_add() {
        assert_eq!(CardSet::from_card(B1).add(CardSet::from_card(B3)).0, 0b101);
    }

    #[test]
    fn test_overlaps_with() {
        let base = CardSet::from_cards(&[B1, B3, B5]);
        assert!(base.overlaps_with(CardSet::from_cards(&[B1, B3, B5])));
        assert!(base.overlaps_with(CardSet::from_cards(&[B5, B6, B7])));
        assert!(!base.overlaps_with(CardSet::from_cards(&[Y1, Y3, Y5])));
    }

    #[test]
    fn test_contains() {
        assert!(CardSet::from_cards(&[B1, B3, B5]).contains(B1));
        assert!(!CardSet::from_cards(&[B1, B3, B5]).contains(B2));
        assert!(!CardSet::from_cards(&[B1, B3, B5]).contains(Y3));
    }

    #[test]
    fn test_get_raw() {
        assert_eq!(CardSet::from_cards(&[B1, B3, B4]).get_raw(), 0b1101);
    }

    #[test]
    fn test_raw_reflexive() {
        let cards = CardSet::from_cards(&[B1, B3, B4]);
        assert_eq!(CardSet::from_raw(cards.get_raw()), cards);
    }

    #[test]
    fn test_get_suit() {
        assert_eq!(CardSet::from_card(B1).get_suit(), suit::BLUE);
        assert_eq!(CardSet::from_card(B4).get_suit(), suit::BLUE);
        assert_eq!(CardSet::from_card(B9).get_suit(), suit::BLUE);
        assert_eq!(CardSet::from_card(Y1).get_suit(), suit::YELLOW);
        assert_eq!(CardSet::from_card(Y4).get_suit(), suit::YELLOW);
        assert_eq!(CardSet::from_card(Y9).get_suit(), suit::YELLOW);
        assert_eq!(CardSet::from_card(M1).get_suit(), suit::MAGENTA);
        assert_eq!(CardSet::from_card(M4).get_suit(), suit::MAGENTA);
        assert_eq!(CardSet::from_card(M9).get_suit(), suit::MAGENTA);
        assert_eq!(CardSet::from_card(G1).get_suit(), suit::GREEN);
        assert_eq!(CardSet::from_card(G4).get_suit(), suit::GREEN);
        assert_eq!(CardSet::from_card(G9).get_suit(), suit::GREEN);
        assert_eq!(CardSet::from_card(R1).get_suit(), suit::ROCKETS);
        assert_eq!(CardSet::from_card(R2).get_suit(), suit::ROCKETS);
        assert_eq!(CardSet::from_card(R3).get_suit(), suit::ROCKETS);
        assert_eq!(CardSet::from_card(R4).get_suit(), suit::ROCKETS);
    }

    #[test]
    fn test_highest_card() {
        assert_eq!(
            CardSet::from_cards(&[B1]).highest_card(),
            CardSet::from_card(B1)
        );
        assert_eq!(
            CardSet::from_cards(&[R4]).highest_card(),
            CardSet::from_card(R4)
        );
        assert_eq!(
            CardSet::from_cards(&[B3]).highest_card(),
            CardSet::from_card(B3)
        );
        assert_eq!(
            CardSet::from_cards(&[B1, B3, B5]).highest_card(),
            CardSet::from_card(B5)
        );
        assert_eq!(
            CardSet::from_cards(&[G1, G3, G5]).highest_card(),
            CardSet::from_card(G5)
        );
        assert_eq!(
            CardSet::from_cards(&[B8, B9, R1]).highest_card(),
            CardSet::from_card(R1)
        );
        assert_eq!(
            CardSet::from_cards(&[R1, R2, R3, R4]).highest_card(),
            CardSet::from_card(R4)
        );
    }

    #[test]
    fn test_is_covered_by() {
        assert!(CardSet::from_cards(&[B1]).is_covered_by(CardSet::from_cards(&[B1])));
        assert!(CardSet::from_cards(&[B1]).is_covered_by(CardSet::from_cards(&[B1, Y2])));
        assert!(CardSet::from_cards(&[B1, B3]).is_covered_by(CardSet::from_cards(&[B1, B2, B3])));
        assert!(!CardSet::from_cards(&[B1, B3]).is_covered_by(CardSet::from_cards(&[B2, B3, B4])));
        assert!(!CardSet::from_cards(&[B1, B3]).is_covered_by(CardSet::from_cards(&[G1, G3])));
    }

    #[test]
    fn test_num_set() {
        assert_eq!(CardSet::EMPTY.num_set(), 0);
        assert_eq!(CardSet::from_cards(&[]).num_set(), 0);
        assert_eq!(CardSet::from_cards(&[B1]).num_set(), 1);
        assert_eq!(CardSet::from_cards(&[B1, G7]).num_set(), 2);
        assert_eq!(CardSet::from_cards(&[B1, G7, Y3, R2, M1]).num_set(), 5);
    }

    #[test]
    fn test_bitwise_ops() {
        let x = CardSet::from_cards(&[B1, B3, B5]);
        let y = CardSet::from_cards(&[B5, B7, B9]);
        assert_eq!(x & y, CardSet::from_cards(&[B5]));
        assert_eq!(x | y, CardSet::from_cards(&[B1, B3, B5, B7, B9]));

        assert!(!x.contains(B2));
        assert!(!x.contains(Y1));
        assert!(!x.contains(G3));
        assert!(!x.contains(M5));
        assert!(!x.contains(R4));
    }
}
