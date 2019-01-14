use std::cmp::Ordering;

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Colour {
    Red, Black
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
pub enum Suit{
    Clubs,
    Hearts,
    Diamonds,
    Spades
}

impl Suit {
    pub fn colour(&self) -> Colour {
        match *self {
            Suit::Clubs | Suit::Spades => Colour::Black,
            Suit::Hearts | Suit::Diamonds => Colour::Red
        }
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct SuitContext <'a> {
    suit: Suit,
    order: &'a [Suit]
}

impl <'a> SuitContext <'a> {
    pub fn new(suit: Suit, order: &'a [Suit]) -> SuitContext <'a> {
        SuitContext { suit, order }
    } 
}

impl <'a> PartialOrd for SuitContext<'a> {
    fn partial_cmp(&self, other: &SuitContext) -> Option<Ordering> {
        if self.order != other.order {
            panic!("Attempt to compare 2 different orders!");
        }

        let self_size = self.order
                            .iter().position(|&s| s == self.suit);
        let other_size = other.order
                            .iter().position(|&s| s == other.suit); 

        Some(self_size.cmp(&other_size))
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
)]
pub enum Rank{
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn colour_can_be_derived_from_suit() {
        assert_eq!(Suit::Clubs.colour(), Colour::Black);
        assert_eq!(Suit::Hearts.colour(), Colour::Red);
        assert_eq!(Suit::Diamonds.colour(), Colour::Red);
        assert_eq!(Suit::Spades.colour(), Colour::Black);
    }

    #[test]
    fn suits_can_be_compared() {
        let order = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &order);
        let hearts = SuitContext::new(Suit::Hearts, &order);

        assert!(hearts > clubs);
    }

    #[test]
    #[should_panic]
    fn suits_cannot_be_compared_across_different_orderings() {
        let order1 = [Suit::Clubs, Suit::Hearts];
        let order2 = [Suit::Hearts, Suit::Clubs];
        let clubs = SuitContext::new(Suit::Clubs, &order1);
        let hearts = SuitContext::new(Suit::Hearts, &order2);

        assert!(hearts > clubs);
    }

    #[test]
    fn suits_can_be_initialised_seperately_and_compared() {
        let order1 = [Suit::Clubs, Suit::Hearts];
        let order2 = [Suit::Clubs, Suit::Hearts];
        let clubs = SuitContext::new(Suit::Clubs, &order1);
        let hearts = SuitContext::new(Suit::Hearts, &order2);

        assert!(hearts > clubs);
    }

    #[test]
    fn suits_can_be_compared_with_different_orders() {
        let order = [Suit::Hearts, Suit::Clubs];
        let clubs = SuitContext::new(Suit::Clubs, &order);
        let hearts = SuitContext::new(Suit::Hearts, &order);

        assert!(hearts < clubs);
    }
}
