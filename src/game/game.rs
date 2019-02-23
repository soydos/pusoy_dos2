use super::Player;
use crate::cards::{Deck, PlayedCard};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    _num_decks: u8,
    _num_jokers: u8,
    players: Vec<Player>,
    _reversals_enabled: bool,
}

impl Game {
    pub fn new(
        _num_decks: u8,
        _num_jokers: u8,
        player_ids: &[String],
        _reversals_enabled: bool,
    ) -> Game {
        let mut deck = Deck::new(_num_decks, _num_jokers);
        deck.shuffle();
        let cards = deck.deal(player_ids.len() as u8);

        let players = cards
            .iter()
            .zip(player_ids)
            .map(|(c, id)| Player::new(id.to_string(), c.clone()))
            .collect();

        Game {
            _num_decks,
            _num_jokers,
            players,
            _reversals_enabled,
        }
    }

    pub fn play_move(&self, _player_id: &str, _player_move: Vec<PlayedCard>) -> Result<(), ()> {
        Err(())
    }

    pub fn get_player(&self, id: &str) -> Option<Player> {
        match self.players.iter().find(|&p| p.get_id() == id) {
            Some(p) => Some(p.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn invalid_player_cannot_make_a_move() {
        let ids = [String::from("a"), String::from("b"), String::from("c")];

        let game = Game::new(1, 0, &ids, false);

        let three_of_clubs = Card::new(Rank::Three, Suit::Clubs);
        let three_of_clubs_hand_card = PlayedCard::new(three_of_clubs, Rank::Three, Suit::Clubs);
        let player_move = vec![three_of_clubs_hand_card];

        let result = game.play_move("INVALID_PLAYER_ID", player_move);

        let expected_result = match result {
            Err(_) => true,
            _ => false,
        };

        assert!(expected_result);
    }

    #[test]
    fn it_allows_retrieving_a_player_by_id() {
        let ids = [String::from("a"), String::from("b"), String::from("c")];
        let game = Game::new(1, 0, &ids, false);

        let player_a = game.get_player("a").unwrap();

        assert_eq!(player_a.get_card_count(), 18);
    }

}
