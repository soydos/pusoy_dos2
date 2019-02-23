use super::{Rank, Suit};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Card {
    Joker,
    Standard(Rank, Suit)
}

impl Card {
    pub fn get_rank(&self) -> Option<Rank> {
        match *self {
            Card::Standard(r, _) => Some(r),
            _                    => None
        }
    }

    pub fn get_suit(&self) -> Option<Suit> {
        match *self {
            Card::Standard(_, s) => Some(s),
            _                    => None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PlayedCard {
    rank: Rank,
    suit: Suit,
    is_joker: bool
}

impl PlayedCard {
    pub fn new(rank: Rank, suit: Suit, is_joker: bool) -> PlayedCard {
        PlayedCard { is_joker, rank, suit }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_is_joker(&self) -> bool {
      self.is_joker
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn card_has_rank_and_suit() {
        let ace_of_spades = Card::Standard(Rank::Ace, Suit::Spades);

        assert_eq!(ace_of_spades.get_rank().unwrap(), Rank::Ace);
        assert_eq!(ace_of_spades.get_suit().unwrap(), Suit::Spades);
    }

    #[test]
    fn played_joker_has_rank_and_suit() {
        let joker_ace_of_spades = PlayedCard::new(Rank::Ace, Suit::Spades, true);

        assert_eq!(joker_ace_of_spades.get_rank(), Rank::Ace);
        assert_eq!(joker_ace_of_spades.get_suit(), Suit::Spades);
    }

}
