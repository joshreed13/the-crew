use crate::card::{suit, CardSet, RawCardSet};
use crate::player::{PlayerIndex, NUM_PLAYERS};

pub type Hands = [CardSet; NUM_PLAYERS];

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
}

impl PlayGenerator {
    pub fn new(hands: &Hands, lead_player: PlayerIndex) -> PlayGenerator {
        let mut obj = PlayGenerator {
            counters: hands.map(PositionCounter::new),
            lead_suit: CardSet::EMPTY,
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
        CardSet::from_raw(self.position as u64)
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
        let overflowed: bool = self.position > RawCardSet::BITS;
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

    #[test]
    fn test_get_trick_winner() {
        use crate::card::Card::*;

        assert_eq!(
            Play {
                cards: CardSet::from_cards(&[B8, B9, Y2]),
                lead_suit: suit::BLUE
            }
            .get_trick_winner(),
            CardSet::from_card(B9)
        );
    }
}
