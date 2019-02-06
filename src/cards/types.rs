use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Colour {
    Red, Black
}

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

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SuitContext {
    name: Suit,
    order: [Suit; 4],
    colour: Colour,
}

impl SuitContext {
    pub fn new(name: Suit, order: [Suit; 4]) -> SuitContext {
        let owned_order = order.clone();
        let colour = name.colour();
        SuitContext { name, colour, order: owned_order }
    } 
}

impl PartialOrd for SuitContext {
    fn partial_cmp(&self, other: &SuitContext) -> Option<Ordering> {
        if self.order != other.order {
            panic!("Attempt to compare 2 different orders!");
        }

        let self_size = self.order
                            .iter().position(|&s| s == self.name);
        let other_size = other.order
                            .iter().position(|&s| s == other.name); 

        Some(self_size.cmp(&other_size))
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    Hash,
    Eq,
    Serialize,
    Deserialize,
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

pub fn get_suit_array(suit_order: [Suit; 4]) -> [SuitContext; 4] {
    [
        SuitContext::new(Suit::Clubs, suit_order),
        SuitContext::new(Suit::Hearts, suit_order),
        SuitContext::new(Suit::Diamonds, suit_order),
        SuitContext::new(Suit::Spades, suit_order),
    ]
}

pub fn get_rank_array() -> [Rank; 13] {
    [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four, 
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ]
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
        let order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];

        let clubs = SuitContext::new(Suit::Clubs, order);
        let hearts = SuitContext::new(Suit::Hearts, order);

        assert!(hearts > clubs);
    }

    #[test]
    #[should_panic]
    fn suits_cannot_be_compared_across_different_orderings() {
        let order1 = [
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs,
            Suit::Spades
        ];
        let order2 = [
            Suit::Hearts,
            Suit::Clubs,
            Suit::Diamonds,
            Suit::Spades
        ];

        let clubs = SuitContext::new(Suit::Clubs, order1);
        let hearts = SuitContext::new(Suit::Hearts, order2);

        assert!(hearts > clubs);
    }

    #[test]
    fn suits_can_be_initialised_seperately_and_compared() {
        let order1 = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let order2 = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, order1);
        let hearts = SuitContext::new(Suit::Hearts, order2);

        assert!(hearts > clubs);
    }

    #[test]
    fn suits_can_be_compared_with_different_orders() {
        let order = [
            Suit::Hearts,
            Suit::Clubs,
            Suit::Diamonds,
            Suit::Spades
        ];
        let clubs = SuitContext::new(Suit::Clubs, order);
        let hearts = SuitContext::new(Suit::Hearts, order);

        assert!(hearts < clubs);
    }

    #[test]
    fn it_exposes_a_convenience_function_for_grabbing_suits() {
        let order = [
            Suit::Hearts,
            Suit::Clubs,
            Suit::Diamonds,
            Suit::Spades
        ];
        let suits = get_suit_array(order);
        let clubs = SuitContext::new(Suit::Clubs, order);
        let hearts = SuitContext::new(Suit::Hearts, order);
        let diamonds = SuitContext::new(Suit::Diamonds, order);
        let spades = SuitContext::new(Suit::Spades, order);

        assert_eq!(suits[0], clubs);
        assert_eq!(suits[1], hearts);
        assert_eq!(suits[2], diamonds);
        assert_eq!(suits[3], spades);
    }
}
