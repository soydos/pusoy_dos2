use super::{
    HandCard,
    Card,
    SuitContext,
    Suit,
    Rank,
    get_rank_array
};

#[derive(Clone)]
pub struct Deck<'a>(Vec<HandCard<'a>>);

impl <'a> Deck<'a> {
    pub fn new(
        number_of_decks: u8,
        number_of_jokers: u8,
        suits: &'a[SuitContext],
    ) -> Deck<'a> {
        let ranks = get_rank_array();
        let mut cards = vec!();

        while cards.len() < number_of_jokers as usize {
            cards.push(
                HandCard::Joker(cards.len() as u32)
            );
        }

        let mut deck_count = 0;

        while deck_count < number_of_decks {
            for suit in suits {
                for rank in &ranks {
                    let card = Card::new(rank.clone(), suit, false);
                    cards.push(HandCard::Card(card));
                }
            }
            deck_count += 1;
        }

        Deck(cards)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn it_can_create_a_simple_52_deck() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];
        let suits = get_suit_array(&suit_order);
        let deck = Deck::new(1, 0, &suits);
        assert_eq!(deck.count(), 52);
    }

    #[test]
    fn it_can_add_jokers() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];
        let suits = get_suit_array(&suit_order);
        let deck = Deck::new(1, 1, &suits);
        assert_eq!(deck.count(), 53);

    }

    #[test]
    fn it_can_do_multiple_decks() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];
        let suits = get_suit_array(&suit_order);
        let deck = Deck::new(2, 0, &suits);
        assert_eq!(deck.count(), 104);

    }

    #[test]
    fn it_can_add_jokers_with_multiple_decks() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];
        let suits = get_suit_array(&suit_order);
        let deck = Deck::new(2, 1, &suits);
        assert_eq!(deck.count(), 105);

    }

}
