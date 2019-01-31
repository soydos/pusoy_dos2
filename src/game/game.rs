use crate::cards::{SuitContext, Deck, PlayedCard};
use super::Player;

pub struct Game<'a> {
    players: Vec<Player<'a>>,
    reversals_enabled: bool,
}

impl <'a> Game<'a> {
    pub fn new(
        num_decks: u8,
        num_jokers: u8,
        player_ids: &'a [String],
        suits: &'a [SuitContext],
        reversals_enabled: bool,
    ) -> Game<'a> {
        let mut deck = Deck::new(num_decks, num_jokers, suits);
        deck.shuffle();

        let cards = deck.deal(player_ids.len() as u8);
        let players = cards.iter().zip(player_ids)
            .map(|(c, id)| Player::new(id.to_string(), c.clone()))
            .collect();

        Game {
            players,
            reversals_enabled,
        }

    }

    fn play_move(&self, player_id: &str, player_move: Vec<PlayedCard>) -> Result<(), ()> {
        Err(())
    }

    fn get_player(&self, id: &str) -> Option<&Player> {
        self.players.iter().find(|&p| p.get_id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn invalid_player_cannot_make_a_move() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];

        let ids = [String::from("a"), String::from("b"), String::from("c")];

        let suits = get_suit_array(&suit_order);
        let game = Game::new(1, 0, &ids, &suits, false);

        let clubs = SuitContext::new(Suit::Clubs, &suit_order);
        let three_of_clubs = Card::new(Rank::Three, &clubs, false);
        let three_of_clubs_hand_card = PlayedCard::new(three_of_clubs, false);
        let player_move = vec!(
            three_of_clubs_hand_card
        );

        let result = game.play_move("INVALID_PLAYER_ID", player_move); 

        let expected_result = match result {
            Err(_) => true,
            _ => false
        };

        assert!(expected_result);
    }

    #[test]
    fn it_allows_retrieving_a_player_by_id() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];

        let ids = [String::from("a"), String::from("b"), String::from("c")];
        let suits = get_suit_array(&suit_order);
        let game = Game::new(1, 0, &ids, &suits, false);

        let player_a = game.get_player("a").unwrap();

        assert_eq!(player_a.get_card_count(), 18);
    }

}

