use crate::card::{card_at_position, get_suit, suit, CardIndex, CardSet};
use crate::player::{PlayerIndex, NUM_PLAYERS};

pub struct Play {
    pub cards: CardSet,
    lead_suit: CardSet,
}

pub fn get_trick_winner(play: &Play) -> CardSet {
    let eligible_cards = play.lead_suit | suit::ROCKETS;
    let relevant_cards = play.cards & eligible_cards;
    get_high_card(relevant_cards)
}

fn get_high_card(cards: CardSet) -> CardSet {
    1 << (CardSet::BITS - cards.leading_zeros())
}

pub struct PlayGenerator {
    counters: [PositionCounter; NUM_PLAYERS],
    lead_suit: CardSet,
}

impl PlayGenerator {
    pub fn new(hands: &[CardSet; NUM_PLAYERS], lead_player: PlayerIndex) -> PlayGenerator {
        let mut obj = PlayGenerator {
            counters: hands.map(PositionCounter::new),
            lead_suit: 0,
        };
        obj.counters.rotate_left(lead_player as usize);
        obj.set_lead_suit();
        obj
    }

    fn build_cards(&self) -> CardSet {
        self.counters
            .iter()
            .fold(0, |a, x| a | card_at_position(x.position))
    }

    fn set_lead_suit(&mut self) {
        self.lead_suit = get_suit(card_at_position(self.counters[0].position));
        for counter in &mut self.counters[1..] {
            let must_follow_suit = counter.hand & self.lead_suit != 0;
            let mask = if must_follow_suit {
                self.lead_suit
            } else {
                CardSet::MAX
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
    hand: CardSet,
    mask: CardSet,
    position: CardIndex,
}

impl PositionCounter {
    fn new(hand: CardSet) -> PositionCounter {
        let initial_pos: CardIndex = hand.trailing_zeros() as CardIndex;
        PositionCounter {
            hand: hand.rotate_right(initial_pos as u32),
            mask: 0,
            position: initial_pos,
        }
    }

    fn increment(&mut self) -> bool {
        self.hand = self.hand.rotate_right(1);
        let next_step = self.hand.trailing_zeros() as u8;
        self.hand = self.hand.rotate_right(next_step as u32);
        self.position += next_step + 1;
        let overflowed: bool = self.position > CardSet::BITS as u8;
        self.position %= CardSet::BITS as u8;
        overflowed
    }

    fn reset_with_mask(&mut self, mask: CardSet) {
        self.mask = mask;
        self.hand = self.hand.rotate_left(self.position as u32);
        self.position = 0;
        self.increment();
    }
}
