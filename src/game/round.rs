use super::{compare_hands, Hand, Player};
use crate::cards::{Card, PlayedCard, Rank, Suit};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum SubmitError {
    FirstRoundPass,
    FirstHandMustBeThreeClubs,
    HandNotHighEnough,
    NotCurrentPlayer,
    InvalidHand,
}

#[derive(Clone, Debug)]
pub struct Round {
    players: Vec<Player>,
    next_player: Option<&'static str>,
    last_move: Option<Hand>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
}

impl Round {
    pub fn new(
        players: Vec<Player>,
        next_player: Option<&'static str>,
        last_move: Option<Hand>,
        suit_order: [Suit; 4],
        rank_order: [Rank; 13],
    ) -> Round {
        Round {
            players,
            next_player,
            last_move,
            suit_order,
            rank_order,
        }
    }

    pub fn get_next_player(&self) -> Option<&str> {
        match self.next_player {
            None => self.get_starting_player(),
            x => x,
        }
    }

    pub fn submit_move(&self, user_id: &str, cards: Vec<PlayedCard>) -> Result<Round, SubmitError> {
        if user_id != self.get_next_player().unwrap_or("invalid_player") {
            return Err(SubmitError::NotCurrentPlayer);
        }

        let hand = Hand::build(cards.clone());
        if hand.is_none() {
            return Err(SubmitError::InvalidHand);
        }

        if self.last_move == None {
            if cards.len() == 0 {
                return Err(SubmitError::FirstRoundPass);
            } else if !self.contains_lowest_card(cards) {
                return Err(SubmitError::FirstHandMustBeThreeClubs);
            }
        } else {
            if !self.hand_beats_last_move(hand.unwrap()) {
                return Err(SubmitError::HandNotHighEnough);
            }
        }

        // todo - return updated Round
        Ok(self.clone())
    }

    fn get_starting_player(&self) -> Option<&str> {
        let lowest_card = Card::Standard {
            suit: self.suit_order[0],
            rank: self.rank_order[0],
        };
        for player in self.players.iter() {
            if player.has_card(&lowest_card) {
                return Some(player.get_id());
            }
        }
        None
    }

    fn hand_beats_last_move(&self, cards: Hand) -> bool {
        compare_hands(
            self.last_move.expect("cannot compare when no last_move"),
            cards,
            self.suit_order,
            self.rank_order,
        )
    }

    fn contains_lowest_card(&self, cards: Vec<PlayedCard>) -> bool {
        for &card in cards.iter() {
            if card.get_rank() == self.rank_order[0] && card.get_suit() == self.suit_order[0] {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    static DEFAULT_SUIT_ORDER: [Suit; 4] =
        [Suit::Clubs, Suit::Hearts, Suit::Diamonds, Suit::Spades];

    static DEFAULT_RANK_ORDER: [Rank; 13] = [
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
    ];

    #[test]
    fn when_game_hasnt_started_player_with_3clubs_starts() {
        let a_cards = vec![Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);

        let players = vec![player_a, player_b];

        let round = Round::new(players, None, None, DEFAULT_SUIT_ORDER, DEFAULT_RANK_ORDER);

        assert_eq!(round.get_next_player(), Some("a"));
    }

    #[test]
    fn when_game_has_started_there_will_be_a_current_player() {
        let a_cards = vec![Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let round = Round::new(
            players,
            Some("b"),
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );

        assert_eq!(round.get_next_player(), Some("b"));
    }

    #[test]
    fn player_cannot_start_a_game_with_a_pass() {
        let a_cards = vec![Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let round = Round::new(players, None, None, DEFAULT_SUIT_ORDER, DEFAULT_RANK_ORDER);

        let err = round.submit_move("a", vec![]).err().unwrap();

        assert_eq!(err, SubmitError::FirstRoundPass);
    }

    #[test]
    fn player_must_start_a_game_with_three_clubs() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let round = Round::new(players, None, None, DEFAULT_SUIT_ORDER, DEFAULT_RANK_ORDER);
        let played_hand = vec![PlayedCard::new(Rank::Six, Suit::Clubs, false)];
        let err = round.submit_move("a", played_hand).err().unwrap();
        assert_eq!(err, SubmitError::FirstHandMustBeThreeClubs);
    }

    #[test]
    fn playing_a_valid_hand_returns_the_new_round() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let round = Round::new(players, None, None, DEFAULT_SUIT_ORDER, DEFAULT_RANK_ORDER);
        let played_hand = vec![PlayedCard::new(Rank::Three, Suit::Clubs, false)];
        assert!(round.submit_move("a", played_hand).is_ok());
    }

    #[test]
    fn lower_hand_cannot_beat_last_move() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let last_move = Some(Hand::Single(PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false,
        )));
        let round = Round::new(
            players,
            Some("a"),
            last_move,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![PlayedCard::new(Rank::Three, Suit::Clubs, false)];

        let err = round.submit_move("a", played_hand).err().unwrap();
        assert_eq!(err, SubmitError::HandNotHighEnough);
    }

    #[test]
    fn higher_hand_can_beat_last_move() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let last_move = Some(Hand::Single(PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false,
        )));
        let round = Round::new(
            players,
            Some("a"),
            last_move,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![PlayedCard::new(Rank::Six, Suit::Clubs, false)];

        assert!(round.submit_move("a", played_hand).is_ok());
    }

    #[test]
    fn invalid_player_cannot_make_a_move() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let last_move = Some(Hand::Single(PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false,
        )));
        let round = Round::new(
            players,
            Some("a"),
            last_move,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![PlayedCard::new(Rank::Six, Suit::Clubs, false)];

        let err = round.submit_move("b", played_hand).err().unwrap();
        assert_eq!(err, SubmitError::NotCurrentPlayer);
    }

    #[test]
    fn it_should_be_a_valid_hand() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let last_move = Some(Hand::Single(PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false,
        )));
        let round = Round::new(
            players,
            Some("a"),
            last_move,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ];

        let err = round.submit_move("a", played_hand).err().unwrap();
        assert_eq!(err, SubmitError::InvalidHand);
    }

    #[test]
    fn it_should_be_a_valid_hand_even_at_start() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let round = Round::new(players, None, None, DEFAULT_SUIT_ORDER, DEFAULT_RANK_ORDER);
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ];

        let err = round.submit_move("a", played_hand).err().unwrap();
        assert_eq!(err, SubmitError::InvalidHand);
    }

    // todo:
    // - player must have card in hand
    // - passing empties the table
    // - removing card from player on submit and updating next player
}
