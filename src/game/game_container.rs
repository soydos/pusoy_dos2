use super::{
    Player,
    Round,
    SubmitError,
    Hand,
    sort_unplayed_cards,
    Ruleset,
    compare_hands
};
use crate::cards::{
    get_rank_array,
    Deck,
    PlayedCard,
    Suit,
    Rank,
};
use crate::ai::get_move;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    num_decks: u8,
    num_jokers: u8,
    round: Round,
    winners: Vec<String>,
    ruleset: Ruleset,
}

impl Game {
    pub fn new(
        num_decks: u8,
        num_jokers: u8,
        player_ids: &[String],
        suit_order: [Suit; 4],
        ruleset: Ruleset
    ) -> Game {
        let rank_order = get_rank_array();

        let mut deck = Deck::new(num_decks, num_jokers);
        deck.shuffle();
        let cards = deck.deal(player_ids.len() as u8);

        let players: Vec<Player> = cards
            .iter()
            .zip(player_ids)
            .map(|(c, id)| {
                let mut player_hand = sort_unplayed_cards(
                    &c, suit_order, rank_order
                );
                player_hand.reverse();

                Player::new(
                    id.to_string(),
                    player_hand
                )
            })
            .collect();

        let round = Round::new(
            players.clone(),
            None,
            None,
            None,
            suit_order,
            rank_order,
            ruleset
        );

        Game {
            num_decks,
            num_jokers,
            round,
            winners: vec!(),
            ruleset
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
                if player.get_hand().is_empty()
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

    pub fn suggest_move(&self, id: &str) -> Option<Vec<PlayedCard>> {
        get_move(
            self.get_last_move(),
            self.get_player(&id),
            self.round.get_suit_order(),
            self.round.get_rank_order(),
        )
            
    }

    pub fn get_winners(&self) -> Vec<String> {
        self.winners.clone()
    }

    pub fn check_move(
        &self,
        hand: Vec<PlayedCard>) -> bool {

        let new_hand_option = Hand::build(hand.clone());
        let last_move_option = self.round.get_last_move();

        if new_hand_option.is_none() {
            return false;
        }

        if last_move_option.is_none() {

            let lowest_card = PlayedCard::new(
                self.round.get_rank_order()[0],
                self.round.get_suit_order()[0],
                false
            );

            return hand.contains(&lowest_card);
        }

        let new_hand = new_hand_option.expect("invalid hand");
        let last_move = last_move_option.expect("no last move");

        if last_move == Hand::Pass {
            return true;
        }

        compare_hands(
            last_move,
            new_hand,
            self.ruleset.flush_precedence,
            self.round.get_suit_order(),
            self.round.get_rank_order()
        )
    }

    pub fn get_suit_order(&self) -> [Suit; 4] {
        self.round.get_suit_order()
    }

    pub fn get_rank_order(&self) -> [Rank; 13] {
        self.round.get_rank_order()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;
    use crate::game::FlushPrecedence;

    const DEFAULT_RULESET: Ruleset = Ruleset{
        reversals_enabled: true,
        flush_precedence: FlushPrecedence::Rank,
    };


    #[test]
    fn it_allows_retrieving_a_player_by_id() {
        let ids = [
            String::from("a"),
            String::from("b"),
            String::from("c")
        ];
        let game = Game::new(
            1, 0, &ids, get_suit_array(), DEFAULT_RULESET
        );
        let player_a = game.get_player("a").unwrap();

        assert_eq!(player_a.get_card_count(), 18);
    }

    #[test]
    fn when_game_hasnt_started_player_with_lowest_card_starts() {
        let ids = [String::from("a"), String::from("b")];
        let game = Game::new(
            1, 0, &ids, get_suit_array(), DEFAULT_RULESET
        );

        let next_player = game.get_next_player().unwrap();
        let three_clubs = Card::Standard {
            deck_id: 0,
            rank: Rank::Three,
            suit: Suit::Clubs,
        };

        assert!(game.get_player(&next_player).unwrap()
            .has_card(three_clubs));
    }

    #[test]
    fn player_loses_cards_that_it_plays() {
        let ids = ["a".to_string(), "b".to_string()];
        let mut game = Game::new(
            1,0, &ids, get_suit_array(), DEFAULT_RULESET
        );

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
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
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
            DEFAULT_RULESET
        );

        let mut game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!(),
            ruleset: DEFAULT_RULESET 
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
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
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
            DEFAULT_RULESET
        );

        let mut game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!(),
            ruleset: DEFAULT_RULESET
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

    #[test]
    fn player_ids_only_appear_in_the_winners_list_once() {
        let a_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];
        let c_cards = vec![];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];
        let round = Round::new(
            players,
            Some("c".to_string()),
            Some(Hand::Pass),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
            DEFAULT_RULESET
        );

        let mut game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!["c".to_string()],
            ruleset: DEFAULT_RULESET
        };

        let hand = vec![];

        let _ = game.play_move("c", hand);

        assert_eq!(game.get_winners().len(), 1);
    }

    #[test]
    fn winners_list_contains_order_of_winners() {
        let a_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];
        let c_cards = vec![];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];
        let round = Round::new(
            players,
            Some("b".to_string()),
            Some(Hand::Pass),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
            DEFAULT_RULESET
        );

        let mut game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!["c".to_string()],
            ruleset: DEFAULT_RULESET
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
            game.get_winners().get(1).unwrap(),
            "b"
        );
    }

    #[test]
    fn check_move_returns_false_when_unable_to_play() {
        let ids = ["a".to_string(), "b".to_string()];
        let game = Game::new(
            1,0, &ids, get_suit_array(), DEFAULT_RULESET
        );

        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Hearts,
                false,
            )
        ];

        let result = game.check_move(hand);

        assert!(!result);
    }

    #[test]
    fn check_move_returns_ok_when_able_to_play() {
        let ids = ["a".to_string(), "b".to_string()];
        let game = Game::new(
            1,0, &ids, get_suit_array(), DEFAULT_RULESET
        );

        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            )
        ];

        let result = game.check_move(hand);

        assert!(result);
    }

    #[test]
    fn check_move_returns_false_when_hand_is_invalid() {
        let ids = ["a".to_string(), "b".to_string()];
        let game = Game::new(
            1,0, &ids, get_suit_array(), DEFAULT_RULESET
        );

        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            ),
            PlayedCard::new(
                Rank::Four,
                Suit::Clubs,
                false,
            )
        ];

        let result = game.check_move(hand);

        assert!(!result);
    }

    #[test]
    fn check_move_returns_true_when_hand_would_play() {
        let a_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];
        let c_cards = vec![];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];
        let round = Round::new(
            players,
            Some("b".to_string()),
            Some(Hand::Pass),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
            DEFAULT_RULESET
        );

        let game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!["c".to_string()],
            ruleset: DEFAULT_RULESET
        };

        let hand = vec![
            PlayedCard::new(
                Rank::Four,
                Suit::Clubs,
                false,
            )
        ];

        let result = game.check_move(hand);

        assert!(result);
    }

    #[test]
    fn check_move_returns_true_when_hand_would_beat_last_move() {
        let a_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];
        let c_cards = vec![];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];
        let round = Round::new(
            players,
            Some("b".to_string()),
            Some(Hand::Single(PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            ))),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
            DEFAULT_RULESET
        );

        let game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!["c".to_string()],
            ruleset: DEFAULT_RULESET
        };

        let hand = vec![
            PlayedCard::new(
                Rank::Four,
                Suit::Clubs,
                false,
            )
        ];

        let result = game.check_move(hand);

        assert!(result);
    }

    #[test]
    fn check_move_returns_false_when_hand_would_beat_last_move() {
        let a_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];
        let c_cards = vec![];

        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];
        let round = Round::new(
            players,
            Some("b".to_string()),
            Some(Hand::Single(PlayedCard::new(
                Rank::Four,
                Suit::Clubs,
                false,
            ))),
            Some("a".to_string()),
            get_suit_array(),
            get_rank_array(),
            DEFAULT_RULESET
        );

        let game = Game{
            num_decks: 1,
            num_jokers: 1,
            round,
            winners: vec!["c".to_string()],
            ruleset: DEFAULT_RULESET
        };

        let hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            )
        ];

        let result = game.check_move(hand);

        assert!(!result);
    }

}
