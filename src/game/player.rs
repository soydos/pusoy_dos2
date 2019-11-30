use crate::cards::{Card, PlayedCard};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerError {
    PlayerDoesntHaveCard,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    id: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(id: String, hand: Vec<Card>) -> Player {
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

    pub fn play_move(&mut self, cards: Vec<PlayedCard>) -> Result<Player, PlayerError> {
        for card in cards.iter() {
            match self.hand.iter()
                .position(|&c| {
                    let played_card = card.to_card();

                    c.get_rank() == played_card.get_rank() &&
                        c.get_suit() == played_card.get_suit()
                }) {

                Some(index) => self.hand.remove(index),
                _ => return Err(PlayerError::PlayerDoesntHaveCard),
            };
        }

        Ok(self.clone())
    }

    pub fn has_card(&self, card: Card) -> bool {
        self.hand.contains(&card)
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

    #[test]
    fn player_has_card() {
        let id = String::from("id1");
        let hand = vec![Card::Standard {
            deck_id: 0,
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];

        let three_clubs = Card::Standard {
            deck_id: 0,
            rank: Rank::Three,
            suit: Suit::Clubs,
        };
        let four_clubs = Card::Standard {
            deck_id: 0,
            rank: Rank::Four,
            suit: Suit::Clubs,
        };

        let player = Player::new(id, hand);

        assert!(player.has_card(three_clubs));
        assert!(!player.has_card(four_clubs));
    }

    #[test]
    fn it_removes_played_cards_from_hand() {
        let id = String::from("id1");
        let hand = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false
        )];

        let remaining_hand = vec![Card::Standard {
            deck_id: 0,
            rank: Rank::Six,
            suit: Suit::Clubs,
        }];

        let mut player = Player::new(id, hand);

        assert!(player.play_move(played_hand).is_ok());
        assert_eq!(player.get_hand(), remaining_hand);
    }

    #[test]
    fn it_errors_if_player_tries_to_play_cards_they_dont_have() {
        let id = String::from("id1");
        let hand = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![PlayedCard::new(
            Rank::Five,
            Suit::Clubs,
            false
        )];

        let mut player = Player::new(id, hand);
        let err = player.play_move(played_hand).err().unwrap();

        assert_eq!(err, PlayerError::PlayerDoesntHaveCard);
    }

    #[test]
    fn played_cards_returns_updated_player() {
        let id = String::from("id1");
        let hand = vec![
            Card::Standard {
                deck_id: 0,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false
        )];

        let remaining_hand = vec![Card::Standard {
            deck_id: 0,
            rank: Rank::Six,
            suit: Suit::Clubs,
        }];

        let mut player = Player::new(id, hand);

        let new_player = player.play_move(played_hand).unwrap();

        assert_eq!(new_player.get_hand(), remaining_hand);
    }

    #[test]
    fn cards_from_any_deck_can_be_played() {
        let id = String::from("id1");
        let hand = vec![
            Card::Standard {
                deck_id: 1,
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                deck_id: 0,
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![PlayedCard::new(
            Rank::Three,
            Suit::Clubs,
            false
        )];

        let mut player = Player::new(id, hand);

        let new_player = player.play_move(played_hand);

        assert!(new_player.is_ok());

    }
}
