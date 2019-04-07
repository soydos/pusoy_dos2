use rand::seq::SliceRandom;

use super::{get_rank_array, get_suit_array, Card};

#[derive(Clone)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(number_of_decks: u8, number_of_jokers: u8) -> Deck {
        let ranks = get_rank_array();
        let suits = get_suit_array();
        let mut cards = vec![];

        while cards.len() < number_of_jokers as usize {
            cards.push(Card::Joker { deck_id: 0 });
        }

        let mut deck_count = 0;

        while deck_count < number_of_decks {
            for suit in &suits {
                for rank in &ranks {
                    let card = Card::Standard {
                        deck_id: deck_count,
                        rank: *rank,
                        suit: *suit,
                    };
                    cards.push(card);
                }
            }
            deck_count += 1;
        }

        Deck(cards)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.0.shuffle(&mut rng);
    }

    pub fn deal(&self, players: u8) -> Vec<Vec<Card>> {
        let mut index = 0;
        let mut deck_stack = self.0.clone();
        let mut dealt_stacks = self.get_nested_vec(players);

        while !deck_stack.is_empty() {
            let card = deck_stack.pop();
            dealt_stacks[index].push(card.unwrap());
            index = self.rotate_index_to_max(index, players);
        }

        dealt_stacks
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn to_vec(&self) -> Vec<Card> {
        self.0.clone()
    }

    fn get_nested_vec<T>(&self, players: u8) -> Vec<Vec<T>> {
        let mut dealt_stacks = vec![];
        while dealt_stacks.len() < players as usize {
            dealt_stacks.push(vec![]);
        }

        dealt_stacks
    }

    fn rotate_index_to_max(&self, index: usize, max: u8) -> usize {
        if index + 1 < max as usize {
            index + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn it_can_create_a_simple_52_deck() {
        let deck = Deck::new(1, 0);
        assert_eq!(deck.count(), 52);
    }

    #[test]
    fn it_can_add_jokers() {
        let deck = Deck::new(1, 1);
        assert_eq!(deck.count(), 53);
    }

    #[test]
    fn it_can_do_multiple_decks() {
        let deck = Deck::new(2, 0);
        assert_eq!(deck.count(), 104);
    }

    #[test]
    fn it_can_add_jokers_with_multiple_decks() {
        let deck = Deck::new(2, 1);
        assert_eq!(deck.count(), 105);
    }

    #[test]
    fn it_can_shuffle() {
        let mut deck = Deck::new(1, 0);

        let original_order = deck.to_vec();

        deck.shuffle();

        let new_order = deck.to_vec();

        let not_deep_equal = original_order
            .iter()
            .zip(new_order)
            .any(|(a, b)| a.clone() != b.clone());
        assert!(not_deep_equal);
    }

    #[test]
    fn it_can_deal() {
        let deck = Deck::new(1, 0);

        let dealt = deck.deal(4);
        assert_eq!(dealt.len(), 4);
        assert_eq!(dealt[0].len(), 13);
    }
}
