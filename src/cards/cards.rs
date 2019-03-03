use super::{Rank, Suit};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Card {
    Joker,
    Standard { rank: Rank, suit: Suit },
}

impl Card {
    pub fn get_rank(&self) -> Option<Rank> {
        match *self {
            Card::Standard { rank, .. } => Some(rank),
            _ => None,
        }
    }

    pub fn get_suit(&self) -> Option<Suit> {
        match *self {
            Card::Standard { suit, .. } => Some(suit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PlayedCard {
    rank: Rank,
    suit: Suit,
    is_joker: bool,
}

impl PlayedCard {
    pub fn new(rank: Rank, suit: Suit, is_joker: bool) -> PlayedCard {
        PlayedCard {
            is_joker,
            rank,
            suit,
        }
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

    pub fn to_card(&self) -> Card {
        if self.is_joker {
            Card::Joker
        } else {
            Card::Standard {
                rank: self.rank,
                suit: self.suit,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn card_has_rank_and_suit() {
        let ace_of_spades = Card::Standard {
            rank: Rank::Ace,
            suit: Suit::Spades,
        };

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
