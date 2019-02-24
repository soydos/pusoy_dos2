use crate::cards::Card;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Player {
    id: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(id: String, unsorted_hand: Vec<Card>) -> Player {
        let mut hand = unsorted_hand.clone();
        hand.sort();
        Player { id, hand }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }

    pub fn get_card_count(&self) -> usize {
        self.hand.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn it_has_an_id() {
        let id = "id1";;
        let player = Player::new(String::from("id1"), vec![]);
        assert_eq!(player.get_id(), id);
    }

    #[test]
    fn it_shows_number_of_cards_left() {
        let id = String::from("id1");
        let deck = Deck::new(1, 0);

        let dealt = deck.deal(4);
        let player = Player::new(id, dealt[0].to_owned());
        assert_eq!(player.get_card_count(), 13);
    }

    #[test]
    fn it_can_return_the_hand() {
        let id = String::from("id1");
        let deck = Deck::new(1, 0);

        let dealt = deck.deal(4);
        let player = Player::new(id, dealt[0].to_owned());
        assert_eq!(player.get_hand().len(), 13);
    }

}
