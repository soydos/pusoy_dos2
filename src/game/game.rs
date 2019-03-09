use super::{Player, Round, SubmitError, Hand};
use crate::cards::{
    get_rank_array,
    get_suit_array,
    Deck,
    PlayedCard
};
use crate::ai::get_move;
use wasm_bindgen::prelude::*;

// todo - suit order is a property of game
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    num_decks: u8,
    num_jokers: u8,
    _reversals_enabled: bool,
    round: Round,
    winners: Vec<String>,
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
            None,
            get_suit_array(), // todo - set when game is setup
            get_rank_array(),
        );

        Game {
            num_decks,
            num_jokers,
            _reversals_enabled,
            round,
            winners: vec!()
        }
    }

    pub fn play_move(
        &mut self,
        player_id: &str,
        player_move: Vec<PlayedCard>,
    ) -> Result<(), SubmitError> {
        match self.round.submit_move(player_id, player_move) {
            Ok(new_round) => {
                let player = new_round.get_player(player_id)
                    .unwrap();

                if player.get_hand().len() == 0
                    && !self.winners
                            .contains(&player_id.to_string()) {
                    self.winners.push(player_id.to_string());
                }
                self.round = new_round;
                Ok(())
            },
            Err(x) => Err(x),
        }
    }

    pub fn get_player(&self, id: &str) -> Option<Player> {
        self.round.get_player(id)
    }

    pub fn get_next_player(&self) -> Option<String> {
        self.round.get_next_player()
    }

    pub fn get_last_move(&self) -> Option<Hand> {
        self.round.get_last_move()
    }

    pub fn get_ai_move(&self) -> Option<Vec<PlayedCard>> {
        let player_id = self.get_next_player()
            .expect("no next player!");
        get_move(
            self.get_last_move(),
            self.get_player(&player_id),
            self.round.get_suit_order(),
            self.round.get_rank_order(),
        )
            
    }

    pub fn get_winners(&self) -> Vec<String> {
        self.winners.clone()
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

        let next_player = game.get_next_player().unwrap();
        let three_clubs = Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        };

        assert!(game.get_player(&next_player).unwrap()
            .has_card(&three_clubs));
    }

    #[test]
    fn player_loses_cards_that_it_plays() {
        let ids = ["a".to_string(), "b".to_string()];
        let mut game = Game::new(1, 0, &ids, false);

        let next_player = game.get_next_player()
            .expect("unable to get next player").to_owned();
        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            )
        ];

        let initial_hand_size = game.get_player(&next_player)
            .expect("unable to get player before move")
            .get_hand().len();

        let _ = game.play_move(&next_player, hand);

        let eventual_hand_size = game.get_player(&next_player)
            .expect("unable to get player after move")
            .get_hand().len();

        assert_eq!(initial_hand_size - 1, eventual_hand_size);
    }

    #[test]
    fn game_returns_winners() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);

        let players = vec![player_a, player_b];
        let round = Round::new(
            players,
            Some("b".to_string()),
            Some(Hand::Pass),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
        );

        let mut game = Game{
            num_decks: 1,
            num_jokers: 1,
            _reversals_enabled: false,
            round,
            winners: vec!(),
        };

        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            )
        ];

        let _ = game.play_move("b", hand);

        assert_eq!(
            game.get_winners().first().expect("no winners!"),
            "b"
        );
    }

    #[test]
    fn player_only_wins_when_it_is_out_of_cards() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);

        let players = vec![player_a, player_b];
        let round = Round::new(
            players,
            Some("b".to_string()),
            Some(Hand::Pass),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
        );

        let mut game = Game{
            num_decks: 1,
            num_jokers: 1,
            _reversals_enabled: false,
            round,
            winners: vec!(),
        };

        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            )
        ];

        let _ = game.play_move("b", hand);

        assert!(game.get_winners().first().is_none());

    }
}
