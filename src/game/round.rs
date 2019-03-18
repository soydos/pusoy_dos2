use super::{compare_hands, Hand, Player};
use crate::cards::{Card, PlayedCard, Rank, Suit};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum SubmitError {
    FirstRoundPass,
    FirstHandMustContainLowestCard,
    HandNotHighEnough,
    NotCurrentPlayer,
    InvalidHand,
    PlayerDoesntHaveCard,
}

#[derive(Clone, Debug)]
pub struct Round {
    players: Vec<Player>,
    next_player: Option<String>,
    last_move: Option<Hand>,
    last_player: Option<String>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
}

impl Round {
    pub fn new(
        players: Vec<Player>,
        next_player: Option<String>,
        last_move: Option<Hand>,
        last_player: Option<String>,
        suit_order: [Suit; 4],
        rank_order: [Rank; 13],
    ) -> Round {
        Round {
            players,
            next_player,
            last_move,
            last_player,
            suit_order,
            rank_order,
        }
    }

    pub fn get_next_player(&self) -> Option<String> {
        match &self.next_player {
            None => {
                if self.get_players_still_in(&self.players).len() > 1 {
                    self.get_starting_player()
                } else {
                    None
                }
            },
            Some(x) => Some(x.to_string()),
        }
    }

    pub fn submit_move(
        &self,
        user_id: &str,
        cards: Vec<PlayedCard>
    ) -> Result<Round, SubmitError> {
        if user_id != self.get_next_player()
            .unwrap_or("invalid_player".to_string()) {
            return Err(SubmitError::NotCurrentPlayer);
        }

        let hand = Hand::build(cards.clone());
        if hand.is_none() {
            return Err(SubmitError::InvalidHand);
        }

        if self.last_move == None {
            if cards.len() == 0 {
                return Err(SubmitError::FirstRoundPass);
            } else if !self.contains_lowest_card(cards.clone()) {
                return Err(
                    SubmitError::FirstHandMustContainLowestCard
                );
            }
        } else if self.last_move != Some(Hand::Pass)
            && hand != Some(Hand::Pass) {
            if !self.hand_beats_last_move(hand.unwrap()) {
                return Err(SubmitError::HandNotHighEnough);
            }
        }

        let mut player = self.get_player(user_id)
            .expect("invalid player!");

        let hand_cards = cards.iter()
            .map(|card| card.to_card())
            .collect();
        
        match player.play_move(hand_cards) {
            Ok(p) => player = p,
            _ => return Err(SubmitError::PlayerDoesntHaveCard)
        }

        let players = self.players.iter().map(|p| {
            if p.get_id() == user_id {
                player.clone()
            } else {
                p.clone()
            }
        }).collect();

        let mut new_last_move = hand;
        let mut new_last_player = Some(user_id.to_string());

        if hand == Some(Hand::Pass) {
            new_last_move = self.last_move;
            new_last_player = self.last_player.to_owned();
        }

        let mut next_player = self.get_next_player_in_rotation(user_id);

        if next_player == new_last_player.clone()
            .unwrap_or("invalid_player".to_string()) {
            new_last_move = Some(Hand::Pass);
        }

        while self.get_player(&next_player)
            .unwrap().get_hand().len() < 1 {

            next_player = self.get_next_player_in_rotation(&next_player);
            if next_player == new_last_player.clone()
                .unwrap_or("invalid_player".to_string()) {
                new_last_move = Some(Hand::Pass);
            }

        }

        let output_next_player = if self.get_players_still_in(&players)
            .len() > 1 {
            Some(next_player)
        } else {
            None
        };

        Ok(Self::new(
            players,
            output_next_player,
            new_last_move,
            new_last_player,
            self.suit_order,
            self.rank_order,
        ))
    }

    pub fn get_player(&self, user_id: &str) -> Option<Player> {
        for player in self.players.iter() {
            if player.get_id() == user_id {
                return Some(player.clone());
            }
        }

        None
    }

    pub fn get_last_move(&self) -> Option<Hand> {
        self.last_move
    }

    pub fn get_last_player(&self) -> Option<String> {
        match &self.last_player {
            None => None,
            Some(x) => Some(x.to_string())
        }
    }

    pub fn get_suit_order(&self) -> [Suit; 4] {
        self.suit_order
    }

    pub fn get_rank_order(&self) -> [Rank; 13] {
        self.rank_order
    }

    fn get_starting_player(&self) -> Option<String> {
        let lowest_card = Card::Standard {
            suit: self.suit_order[0],
            rank: self.rank_order[0],
        };
        for player in self.players.iter() {
            if player.has_card(&lowest_card) {
                return Some(player.get_id().to_string());
            }
        }
        None
    }


    fn hand_beats_last_move(&self, cards: Hand) -> bool {
        compare_hands(
            self.last_move
                .expect("cannot compare when no last_move"),
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

    fn get_next_player_in_rotation(&self, user_id: &str) -> String {
        if self.players.last()
            .unwrap().get_id() == user_id {
            return self.players.first()
                .unwrap().get_id().to_string();
        }
        let mut i = 0;
        let mut index = 0;
        for player in &self.players {
            i = i + 1; 
            if player.get_id() == user_id {
                index = i;
            }
        }

        self.players.get(index).unwrap().get_id().to_string()
    }

    fn get_players_still_in(&self, players: &Vec<Player>) -> Vec<Player> {
        players.iter()
            .filter(|p| p.get_hand().len() > 0)
            .map(|p| p.clone())
            .collect()
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

        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        assert_eq!(round.get_next_player(), Some("a".to_string()));
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
            Some("b".to_string()),
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );

        assert_eq!(round.get_next_player(), Some("b".to_string()));
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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false)
        ];
        let err = round.submit_move("a", played_hand)
            .err().unwrap();
        assert_eq!(
            err,
            SubmitError::FirstHandMustContainLowestCard
        );
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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];
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
            Some("a".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];

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
            Some("a".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false)
        ];

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
            Some("a".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false)
        ];

        let err = round.submit_move("b", played_hand)
            .err().unwrap();
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
            Some("a".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ];

        let err = round.submit_move("a", played_hand)
            .err().unwrap();
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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ];

        let err = round.submit_move("a", played_hand)
            .err().unwrap();
        assert_eq!(err, SubmitError::InvalidHand);
    }

    #[test]
    fn player_cannot_play_cards_it_doesnt_hold() {
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
            Some("a".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
        ];

        let err = round.submit_move("a", played_hand)
            .err().unwrap();

        assert_eq!(err, SubmitError::PlayerDoesntHaveCard);

    }

    #[test]
    fn player_can_only_play_a_card_once() {
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
        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );
        let round = Round::new(
            players,
            Some("a".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
        ];

        let err = round.submit_move("a", played_hand)
            .err().unwrap();

        assert_eq!(err, SubmitError::PlayerDoesntHaveCard);

    }

    #[test]
    fn playing_a_valid_card_removes_from_players_hand() {
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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];

        let new_round = round.submit_move("a", played_hand)
            .unwrap();

        let new_player_a = new_round.get_player("a").unwrap();

        assert_eq!(new_player_a.get_hand().len(), 1);
    }

    #[test]
    fn a_valid_move_is_set_as_last_move() {
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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];

        let new_round = round.submit_move("a", played_hand)
            .unwrap();

        assert_eq!(
            new_round.get_last_move(),
            Some(Hand::Single(PlayedCard::new(
                Rank::Three, Suit::Clubs, false
            )))
        );
    }

    #[test]
    fn when_a_valid_move_is_made_the_next_player_rotates() {
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
        let round = Round::new(
            players,
            None,
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];

        let new_round = round.submit_move("a", played_hand)
            .unwrap();

        assert_eq!(
            new_round.get_next_player(),
            Some("b".to_string())
        );

    }

    #[test]
    fn player_rotation_comes_back_round() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            }
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let players = vec![player_a, player_b];
        let round = Round::new(
            players,
            Some("b".to_string()),
            None,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];

        let new_round = round.submit_move("b", played_hand)
            .unwrap();

        assert_eq!(
            new_round.get_next_player(),
            Some("a".to_string())
        );

    }

    #[test]
    fn passing_moves_without_changing_the_last_move() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            }
        ];
        let c_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            }
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);
        let players = vec![player_a, player_b, player_c];
        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("b".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![];

        let new_round = round.submit_move("b", played_hand)
            .unwrap();

        assert_eq!(
            new_round.get_next_player(),
            Some("c".to_string())
        );

        assert_eq!(
            new_round.get_last_move(),
            last_move
        );
    }

    #[test]
    fn a_valid_move_switches_the_last_player() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Spades,
            }
        ];
        let c_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            }
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);
        let players = vec![player_a, player_b, player_c];
        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("b".to_string()),
            last_move,
            None,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            ),
            PlayedCard::new(
                Rank::Three,
                Suit::Spades,
                false,
            )
        ];

        let new_round = round.submit_move("b", played_hand)
            .unwrap();

        assert_eq!(
            new_round.get_last_player(),
            Some("b".to_string())
        );
    }

    #[test]
    fn if_last_and_next_player_are_same_the_table_is_cleared() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];
        let b_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Spades,
            }
        ];
        let c_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            }
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);
        let players = vec![player_a, player_b, player_c];
        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("b".to_string()),
            last_move,
            Some("c".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );
        let played_hand = vec![];

        let new_round = round.submit_move("b", played_hand)
            .unwrap();

        assert_eq!(
            new_round.get_last_move(),
            Some(Hand::Pass)
        );

    }

    #[test]
    fn any_card_beats_a_pass() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![Card::Standard {
            rank: Rank::Four,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);

        let players = vec![player_a, player_b];

        let round = Round::new(
            players,
            Some("a".to_string()),
            Some(Hand::Pass),
            Some("a".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        let played_hand = vec![
            PlayedCard::new(
                Rank::Three,
                Suit::Clubs,
                false,
            ),
        ];

        let new_round = round.submit_move("a", played_hand);

        assert!(new_round.is_ok());
    }

    #[test]
    fn players_with_no_cards_are_skipped() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![];
        let c_cards = vec![Card::Standard{
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];

        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("a".to_string()),
            last_move,
            Some("c".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        let played_hand = vec![];

        let new_round = round.submit_move(
            "a",
            played_hand
        ).unwrap();

        assert_eq!(
            new_round.get_next_player().unwrap(),
            "c".to_string()
        );
    }

    #[test]
    fn once_the_game_is_over_the_next_player_is_none() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Spades,
            },
            Card::Standard {
                rank: Rank::Four,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![];
        let c_cards = vec![Card::Standard{
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];

        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("a".to_string()),
            last_move,
            Some("c".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        let played_hand = vec![
            PlayedCard::new(
                Rank::Four,
                Suit::Spades,
                false,
            ),
            PlayedCard::new(
                Rank::Four,
                Suit::Clubs,
                false,
            )
        ];

        let new_round = round.submit_move(
            "a",
            played_hand
        ).unwrap();

        assert!(
            new_round.get_next_player().is_none(),
        );

    }

    #[test]
    fn when_player_wins_next_player_starts() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![];
        let c_cards = vec![Card::Standard{
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];

        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("a".to_string()),
            last_move,
            Some("b".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        let played_hand = vec![];

        let new_round = round.submit_move(
            "a",
            played_hand
        ).unwrap();

        assert_eq!(
            new_round.get_last_move().unwrap(),
            Hand::Pass
        );
    }

    #[test]
    fn playing_on_pass_sets_player_as_last_move() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![];
        let c_cards = vec![Card::Standard{
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);

        let players = vec![player_a, player_b, player_c];

        let last_move = Some(
            Hand::Pass
        );

        let round = Round::new(
            players,
            Some("a".to_string()),
            last_move,
            Some("c".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        let played_hand = vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ];

        let new_round = round.submit_move(
            "a",
            played_hand
        ).unwrap();

        assert_eq!(
            new_round.get_last_player().unwrap(),
            "a".to_string()
        );
    }

    #[test]
    fn when_there_are_two_players_left_pass_clears_the_table() {
        let a_cards = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            }
        ];
        let b_cards = vec![];
        let c_cards = vec![];
        let d_cards = vec![
             Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);
        let player_c = Player::new("c".to_string(), c_cards);
        let player_d = Player::new("d".to_string(), d_cards);

        let players = vec![
            player_a,
            player_b,
            player_c,
            player_d
        ];

        let last_move = Some(
            Hand::Pair(
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                ),
                PlayedCard::new(
                    Rank::Three,
                    Suit::Clubs,
                    false,
                )
            )
        );

        let round = Round::new(
            players,
            Some("a".to_string()),
            last_move,
            Some("d".to_string()),
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER
        );

        let played_hand = vec![];

        let new_round = round.submit_move(
            "a",
            played_hand
        ).unwrap();

        assert_eq!(
            new_round.get_last_move().unwrap(),
            Hand::Pass
        );
    }

}
