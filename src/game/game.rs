use super::{Player, Round, SubmitError};
use crate::cards::{get_rank_array, get_suit_array, Deck, PlayedCard};
use wasm_bindgen::prelude::*;

// todo - suit order is a property of game
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    num_decks: u8,
    num_jokers: u8,
    players: Vec<Player>,
    _reversals_enabled: bool,
    round: Round,
}

impl Game {
    pub fn new(
        num_decks: u8,
        num_jokers: u8,
        player_ids: &[String],
        _reversals_enabled: bool,
    ) -> Game {
        let mut deck = Deck::new(num_decks, num_jokers);
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
            get_suit_array(), // todo - set when game is setup
            get_rank_array(),
        );

        Game {
            num_decks,
            num_jokers,
            players,
            _reversals_enabled,
            round,
        }
    }

    pub fn play_move(
        &mut self,
        player_id: &str,
        player_move: Vec<PlayedCard>,
    ) -> Result<(), SubmitError> {
        match self.round.submit_move(player_id, player_move) {
            Ok(new_round) => {
                self.players = self.players.iter().map(|p| {
                    if p.get_id() == player_id {
                        new_round.get_player(player_id)
                            .unwrap_or(p.clone())
                    } else {
                        p.clone()
                    }
                }).collect();
                Ok(())
            },
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
    fn when_game_hasnt_started_player_with_lowest_card_starts() {
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

    #[test]
    fn player_loses_cards_that_it_plays() {
        let ids = [String::from("a"), String::from("b")];
        let mut game = Game::new(1, 0, &ids, false);

        let next_player = game.get_next_player()
            .unwrap().to_owned();
        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            )
        ];

        let initial_hand_size = game.get_player(&next_player)
            .unwrap().get_hand().len();

        let _ = game.play_move(&next_player, hand);

        let eventual_hand_size = game.get_player(&next_player)
            .unwrap().get_hand().len();

        assert_eq!(initial_hand_size - 1, eventual_hand_size);
    }

}
