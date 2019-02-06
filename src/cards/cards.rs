use super::{SuitContext, Rank};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub struct Card {
    rank: Rank,
    suit: SuitContext,
    reversed: bool,
}

impl Card {
    pub fn new(
        rank: Rank,
        suit: SuitContext,
        reversed: bool,
    ) -> Card {
        Card { rank, suit, reversed }
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        if self.reversed != other.reversed {
            panic!("Cannot compare cards with different reversal status");
        }

        let (a, b) = match self.reversed {
            true => (other, self),
            false => (self, other),
        };

        match a.rank.partial_cmp(&b.rank) {
            Some(Ordering::Equal) => a.suit.partial_cmp(&b.suit),
            x => x,
        }
    }
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Debug,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum HandCard {
    Card(Card),
    Joker(u32),    
}


#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub struct PlayedCard {
    card: Card,
    joker: bool
}

impl PlayedCard {
    pub fn new(card: Card, joker: bool) -> PlayedCard {
        PlayedCard{ card, joker }
    }

    pub fn get_rank(&self) -> Rank {
        self.card.get_rank()
    }
}


#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn cards_can_be_compared_based_on_rank() {
        let reversed = false;
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, suit_order);
        let three_of_clubs = Card::new(
            Rank::Three, clubs, reversed
        );
        let four_of_clubs = Card::new(
            Rank::Four, clubs, reversed
        );

        assert!(three_of_clubs < four_of_clubs);
    }

    #[test]
    fn cards_can_be_compared_when_reversed() {
        let reversed = true;
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];

        let clubs = SuitContext::new(Suit::Clubs, suit_order);

        let three_of_clubs = Card::new(Rank::Three, clubs, reversed);
        let four_of_clubs = Card::new(Rank::Four, clubs, reversed);

        assert!(three_of_clubs > four_of_clubs);
    }

    #[test]
    #[should_panic]
    fn cards_cannot_be_compared_across_reversal_status() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, suit_order);

        let three_of_clubs = Card::new(Rank::Three, clubs, false);
        let four_of_clubs = Card::new(Rank::Four, clubs, true);

        // the status of the first card dictates the comparison
        // so this would be correct
        assert!(three_of_clubs < four_of_clubs);
    }

    #[test]
    fn cards_can_be_compared_by_suit() {
        let reversed = false;
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, suit_order);
        let hearts = SuitContext::new(Suit::Hearts, suit_order);
        let three_of_clubs = Card::new(
            Rank::Three, clubs, reversed
        );
        let three_of_hearts = Card::new(
            Rank::Three, hearts, reversed
        );

        assert!(three_of_hearts > three_of_clubs);
    }

    #[test]
    fn cards_can_be_compared_by_suit_when_reversed() {
        let reversed = true;
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, suit_order);
        let hearts = SuitContext::new(Suit::Hearts, suit_order);
        let three_of_clubs = Card::new(
            Rank::Three, clubs, reversed
        );
        let three_of_hearts = Card::new(
            Rank::Three, hearts, reversed
        );
        assert!(three_of_hearts < three_of_clubs);
    }

    #[test]
    fn rank_takes_precedence() {
        let reversed = false;
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, suit_order);
        let hearts = SuitContext::new(Suit::Hearts, suit_order);
        let four_of_clubs = Card::new(Rank::Four, clubs, reversed);
        let three_of_hearts = Card::new(Rank::Three, hearts, reversed);

        assert!(three_of_hearts < four_of_clubs);
    }

}

