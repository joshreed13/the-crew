use crate::card::{suit, CardSet, RawCardSet};
use crate::player::{PlayerIndex, NUM_PLAYERS};

pub type Hands = [CardSet; NUM_PLAYERS];

#[derive(Debug, PartialEq)]
pub struct Play {
    cards: CardSet,
    lead_suit: CardSet,
}

impl Play {
    pub fn get_trick_winner(&self) -> CardSet {
        let eligible_cards = self.lead_suit | suit::ROCKETS;
        let relevant_cards = self.cards & eligible_cards;
        relevant_cards.highest_card()
    }

    pub fn cards(&self) -> CardSet {
        self.cards
    }
}

pub struct PlayGenerator {
    counters: [PositionCounter; NUM_PLAYERS],
    lead_suit: CardSet,
    first_time: bool,
}

impl PlayGenerator {
    pub fn new(hands: &Hands, lead_player: PlayerIndex) -> PlayGenerator {
        let mut obj = PlayGenerator {
            counters: hands.map(PositionCounter::new),
            lead_suit: CardSet::EMPTY,
            first_time: true,
        };
        obj.counters.rotate_left(lead_player as usize);
        obj.set_lead_suit();
        obj
    }

    fn build_cards(&self) -> CardSet {
        self.counters
            .iter()
            .fold(CardSet::EMPTY, |a, x| a | x.get_card())
    }

    fn set_lead_suit(&mut self) {
        self.lead_suit = self.counters[0].get_card().get_suit();
        for counter in &mut self.counters[1..] {
            let must_follow_suit = self
                .lead_suit
                .overlaps_with(CardSet::from_raw(counter.get_hand()));
            let mask = if must_follow_suit {
                self.lead_suit
            } else {
                CardSet::EVERYTHING
            };
            counter.reset_with_mask(mask);
        }
    }

    fn get_play(&self) -> Play {
        Play {
            cards: self.build_cards(),
            lead_suit: self.lead_suit,
        }
    }
}

impl Iterator for PlayGenerator {
    type Item = Play;

    fn next(&mut self) -> Option<Play> {
        if self.first_time {
            self.first_time = false;
            return Some(self.get_play());
        }

        for counter in &mut self.counters[1..].iter_mut().rev() {
            let overflowed = counter.increment();
            if !overflowed {
                return Some(self.get_play());
            }
        }

        let overflowed = self.counters[0].increment();
        if overflowed {
            None
        } else {
            self.set_lead_suit();
            Some(self.get_play())
        }
    }
}

struct PositionCounter {
    hand: RawCardSet,
    mask: RawCardSet,
    position: u32,
}

impl PositionCounter {
    fn new(hand: CardSet) -> PositionCounter {
        let raw_hand = hand.get_raw();
        let initial_pos = raw_hand.trailing_zeros();
        PositionCounter {
            hand: raw_hand.rotate_right(initial_pos),
            mask: RawCardSet::MAX,
            position: initial_pos,
        }
    }

    fn get_card(&self) -> CardSet {
        CardSet::from_bit_index(self.position)
    }

    fn get_hand(&self) -> RawCardSet {
        self.hand.rotate_left(self.position)
    }

    fn increment(&mut self) -> bool {
        self.hand = self.hand.rotate_right(1);
        self.mask = self.mask.rotate_right(1);
        let next_step = (self.hand & self.mask).trailing_zeros();
        self.hand = self.hand.rotate_right(next_step);
        self.mask = self.mask.rotate_right(next_step);
        self.position += next_step + 1;
        let overflowed: bool = self.position >= RawCardSet::BITS;
        self.position %= RawCardSet::BITS;
        overflowed
    }

    fn reset_with_mask(&mut self, mask: CardSet) {
        self.mask = mask.get_raw();
        self.hand = self.hand.rotate_left(self.position);
        self.position = 0;
        self.increment();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card::*;

    #[test]
    fn test_get_trick_winner() {
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B8, B9, Y2]),
                lead_suit: suit::BLUE
            }
            .get_trick_winner(),
            CardSet::from_card(B9)
        );
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B8, B9, Y2]),
                lead_suit: suit::YELLOW
            }
            .get_trick_winner(),
            CardSet::from_card(Y2)
        );
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B8, B9, Y2, R2]),
                lead_suit: suit::BLUE
            }
            .get_trick_winner(),
            CardSet::from_card(R2)
        );
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B8, Y2, R2, R3]),
                lead_suit: suit::BLUE
            }
            .get_trick_winner(),
            CardSet::from_card(R3)
        );
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B8, Y2, R3]),
                lead_suit: suit::ROCKETS
            }
            .get_trick_winner(),
            CardSet::from_card(R3)
        );
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B1, Y9, M9, G9]),
                lead_suit: suit::BLUE
            }
            .get_trick_winner(),
            CardSet::from_card(B1)
        );
        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[G1, G2, G3, G4]),
                lead_suit: suit::GREEN
            }
            .get_trick_winner(),
            CardSet::from_card(G4)
        );
    }

    #[test]
    fn test_position_counter() {
        let cards = CardSet::from_cards(&[B3, B5, B9, Y2, G4, R1]);
        let mut pc = PositionCounter::new(cards);

        assert_eq!(pc.get_card(), CardSet::from_card(B3));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B5));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B9));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(Y2));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(G4));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(R1));
        assert!(pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B3));

        pc.reset_with_mask(suit::BLUE);
        assert_eq!(pc.get_card(), CardSet::from_card(B3));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B5));
        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B9));
        assert!(pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B3));

        assert!(!pc.increment());
        assert_eq!(pc.get_card(), CardSet::from_card(B5));
        assert_eq!(pc.get_hand(), cards.get_raw());
    }

    #[test]
    fn test_play_generator() {
        let play = |suit, cards| Play {
            cards: CardSet::from_cards(cards),
            lead_suit: suit,
        };

        let hands: Hands = [
            CardSet::from_cards(&[B1, M1, G1]),
            CardSet::from_cards(&[B2, G2, R4]),
            CardSet::from_cards(&[Y3, M3, G3]),
            CardSet::from_cards(&[B4, B5, M4]),
        ];

        let expected = vec![
            play(suit::BLUE, &[B1, B2, Y3, B4]),
            play(suit::BLUE, &[B1, B2, Y3, B5]),
            play(suit::BLUE, &[B1, B2, M3, B4]),
            play(suit::BLUE, &[B1, B2, M3, B5]),
            play(suit::BLUE, &[B1, B2, G3, B4]),
            play(suit::BLUE, &[B1, B2, G3, B5]),
            play(suit::MAGENTA, &[M1, B2, M3, M4]),
            play(suit::MAGENTA, &[M1, G2, M3, M4]),
            play(suit::MAGENTA, &[M1, R4, M3, M4]),
            play(suit::GREEN, &[G1, G2, G3, B4]),
            play(suit::GREEN, &[G1, G2, G3, B5]),
            play(suit::GREEN, &[G1, G2, G3, M4]),
        ];

        let pg = PlayGenerator::new(&hands, 0);
        assert_eq!(pg.take(26).collect::<Vec<Play>>(), expected);
    }

    #[test]
    fn test_play_generator_different_leader() {
        let play = |suit, cards| Play {
            cards: CardSet::from_cards(cards),
            lead_suit: suit,
        };

        let hands: Hands = [
            CardSet::from_cards(&[B1, Y1]),
            CardSet::from_cards(&[B2, Y2]),
            CardSet::from_cards(&[Y3, Y7]),
            CardSet::from_cards(&[B4, Y4]),
        ];

        let expected = vec![
            play(suit::YELLOW, &[Y1, Y2, Y3, Y4]),
            play(suit::YELLOW, &[Y1, Y2, Y7, Y4]),
        ];

        let pg = PlayGenerator::new(&hands, 2);
        assert_eq!(pg.take(26).collect::<Vec<Play>>(), expected);
    }
}
