use super::{Rank, Suit};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PlayedCard {
    card: Card,
    rank: Rank,
    suit: Suit,
}

impl PlayedCard {
    pub fn new(card: Card, rank: Rank, suit: Suit) -> PlayedCard {
        PlayedCard { card, rank, suit }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_card(&self) -> Card {
      self.card
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn card_has_rank_and_suit() {
        let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);

        assert_eq!(ace_of_spades.get_rank(), Rank::Ace);
        assert_eq!(ace_of_spades.get_suit(), Suit::Spades);
    }

    #[test]
    fn played_joker_has_rank_and_suit() {
        let joker = Card::new(Rank::Joker, Suit::Joker);
        let joker_ace_of_spades = PlayedCard::new(joker, Rank::Ace, Suit::Spades);

        assert_eq!(joker_ace_of_spades.get_rank(), Rank::Ace);
        assert_eq!(joker_ace_of_spades.get_suit(), Suit::Spades);
        assert_eq!(joker_ace_of_spades.get_card().get_rank(), Rank::Joker);
        assert_eq!(joker_ace_of_spades.get_card().get_suit(), Suit::Joker);
    }

    #[test]
    fn cards_can_be_compared() {
        let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
        let three_of_hearts = Card::new(Rank::Three, Suit::Hearts);
        let two_of_spades = Card::new(Rank::Two, Suit::Spades);
        
        assert!(two_of_spades > three_of_hearts);
        assert!(three_of_hearts > three_of_clubs);
    }
}
