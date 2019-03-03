use crate::cards::Card;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Serialize)]
pub enum PlayerError {
    PlayerDoesntHaveCard,
}

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

    pub fn play_move(&mut self, cards: Vec<Card>) -> Result<Player, PlayerError> {
        for card in cards.iter() {
            match self.hand.iter().position(|&c| c == *card) {
                Some(index) => self.hand.remove(index),
                _ => return Err(PlayerError::PlayerDoesntHaveCard),
            };
        }

        Ok(self.clone())
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
    fn it_removes_played_cards_from_hand() {
        let id = String::from("id1");
        let hand = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];

        let remaining_hand = vec![Card::Standard {
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
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![Card::Standard {
            rank: Rank::Five,
            suit: Suit::Clubs,
        }];

        let mut player = Player::new(id, hand);
        let err = player.play_move(played_hand).err().unwrap();

        assert_eq!(err, PlayerError::PlayerDoesntHaveCard);
    }

    #[test]
    fn played_cards_returns_updated_player() {
        let id = String::from("id1");
        let hand = vec![
            Card::Standard {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card::Standard {
                rank: Rank::Six,
                suit: Suit::Clubs,
            },
        ];

        let played_hand = vec![Card::Standard {
            rank: Rank::Three,
            suit: Suit::Clubs,
        }];

        let remaining_hand = vec![Card::Standard {
            rank: Rank::Six,
            suit: Suit::Clubs,
        }];

        let mut player = Player::new(id, hand);

        let new_player = player.play_move(played_hand).unwrap();

        assert_eq!(new_player.get_hand(), remaining_hand);
    }
}
