use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Colour {
    Red,
    Black,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

impl Suit {
    pub fn colour(&self) -> Colour {
        match *self {
            Suit::Clubs | Suit::Spades => Colour::Black,
            Suit::Hearts | Suit::Diamonds => Colour::Red,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rank {
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
    Two,
}

pub fn get_suit_array() -> [Suit; 4] {
    [Suit::Clubs, Suit::Hearts, Suit::Diamonds, Suit::Spades]
}

pub fn get_rank_array() -> [Rank; 13] {
    [
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
        Rank::Ace,
        Rank::Two,
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
}
