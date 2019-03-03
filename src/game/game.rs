use super::{Player, Round, SubmitError};
use crate::cards::{get_rank_array, get_suit_array, Deck, PlayedCard};
use wasm_bindgen::prelude::*;

// todo - suit order is a property of game
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    _num_decks: u8,
    _num_jokers: u8,
    players: Vec<Player>,
    _reversals_enabled: bool,
    round: Round,
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

        let players: Vec<Player> = cards
            .iter()
            .zip(player_ids)
            .map(|(c, id)| Player::new(id.to_string(), c.clone()))
            .collect();

        let round = Round::new(
            players.clone(),
            None,
            None,
            get_suit_array(),
            get_rank_array(),
        );

        Game {
            _num_decks,
            _num_jokers,
            players,
            _reversals_enabled,
            round,
        }
    }

    pub fn play_move(
        &self,
        player_id: &str,
        player_move: Vec<PlayedCard>,
    ) -> Result<(), SubmitError> {
        match self.round.submit_move(player_id, player_move) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }

    pub fn get_player(&self, id: &str) -> Option<Player> {
        match self.players.iter().find(|&p| p.get_id() == id) {
            Some(p) => Some(p.clone()),
            _ => None,
        }
    }

    pub fn get_next_player(&self) -> Option<&str> {
        self.round.get_next_player()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn it_allows_retrieving_a_player_by_id() {
        let ids = [String::from("a"), String::from("b"), String::from("c")];
        let game = Game::new(1, 0, &ids, false);

        let player_a = game.get_player("a").unwrap();

        assert_eq!(player_a.get_card_count(), 18);
    }

    #[test]
    fn when_game_hasnt_started_player_with_3clubs_starts() {
        let ids = [String::from("a"), String::from("b")];
        let game = Game::new(1, 0, &ids, false);

        let player_a = game.get_player("a").unwrap();

        let next_player = game.get_next_player().unwrap();
        let three_clubs = Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        };

        if player_a.has_card(&three_clubs) {
            assert_eq!(next_player, "a");
        } else {
            assert_eq!(next_player, "b");
        }
    }
}
