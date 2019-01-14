use super::{SuitContext, Rank};
use std::cmp::Ordering;

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Card<'a> {
    rank: Rank,
    suit: &'a SuitContext<'a>,
    reversed: bool,
}

impl <'a> Card<'a> {
    pub fn new(
        rank: Rank,
        suit: &'a SuitContext,
        reversed: bool,
    ) -> Card<'a> {
        Card { rank, suit, reversed }
    }
}

impl <'a> PartialOrd for Card<'a> {
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

#[derive(Clone)]
pub enum HandCard<'a> {
    Card(Card<'a>),
    Joker(u32),    
}

/*
pub struct PlayedCard {
    card: Card
    joker: bool
}
*/

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn cards_can_be_compared_based_on_rank() {
        let reversed = false;
        let suit_order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &suit_order);

        let three_of_clubs = Card::new(Rank::Three, &clubs, reversed);
        let four_of_clubs = Card::new(Rank::Four, &clubs, reversed);

        assert!(three_of_clubs < four_of_clubs);
    }

    #[test]
    fn cards_can_be_compared_when_reversed() {
        let reversed = true;
        let suit_order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &suit_order);

        let three_of_clubs = Card::new(Rank::Three, &clubs, reversed);
        let four_of_clubs = Card::new(Rank::Four, &clubs, reversed);

        assert!(three_of_clubs > four_of_clubs);
    }

    #[test]
    #[should_panic]
    fn cards_cannot_be_compared_across_reversal_status() {
        let suit_order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &suit_order);

        let three_of_clubs = Card::new(Rank::Three, &clubs, false);
        let four_of_clubs = Card::new(Rank::Four, &clubs, true);

        // the status of the first card dictates the comparison
        // so this would be correct
        assert!(three_of_clubs < four_of_clubs);
    }

    #[test]
    fn cards_can_be_compared_by_suit() {
        let reversed = false;
        let suit_order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &suit_order);
        let hearts = SuitContext::new(Suit::Hearts, &suit_order);
        let three_of_clubs = Card::new(Rank::Three, &clubs, reversed);
        let three_of_hearts = Card::new(Rank::Three, &hearts, reversed);

        assert!(three_of_hearts > three_of_clubs);
    }

    #[test]
    fn cards_can_be_compared_by_suit_when_reversed() {
        let reversed = true;
        let suit_order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &suit_order);
        let hearts = SuitContext::new(Suit::Hearts, &suit_order);
        let three_of_clubs = Card::new(Rank::Three, &clubs, reversed);
        let three_of_hearts = Card::new(Rank::Three, &hearts, reversed);

        assert!(three_of_hearts < three_of_clubs);
    }

    #[test]
    fn rank_takes_precedence() {
        let reversed = false;
        let suit_order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &suit_order);
        let hearts = SuitContext::new(Suit::Hearts, &suit_order);
        let four_of_clubs = Card::new(Rank::Four, &clubs, reversed);
        let three_of_hearts = Card::new(Rank::Three, &hearts, reversed);

        assert!(three_of_hearts < four_of_clubs);
    }

}

