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
        num_players: u8,
        suits: &'a [SuitContext],
        reversals_enabled: bool,
    ) -> (Game<'a>, Vec<String>) {
        let mut deck = Deck::new(num_decks, num_jokers, suits);
        deck.shuffle();

        let cards = deck.deal(num_players);
        let players = cards.iter()
            .map(|c| Player::new(nanoid::simple(), c.clone()))
            .collect();
        let game = Game {
            players,
            reversals_enabled,
        };
        let player_ids = game.get_player_ids();

        (
            game,
            player_ids,
        )
    }

    fn get_player_ids(&self) -> Vec<String> {
        self.players.iter()
            .map(|p| String::from(p.get_id()))
            .collect()
    }

    fn play_move(&self, player_id: &str, player_move: Vec<PlayedCard>) -> Result<(), ()> {
        Err(())
    }

    fn get_player(&self, id: String) -> Player {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn it_returns_player_ids_on_creation() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];

        let suits = get_suit_array(&suit_order);
        let (game, ids) = Game::new(1, 0, 3, &suits, false);
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn invalid_player_cannot_make_a_move() {
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];

        let suits = get_suit_array(&suit_order);
        let (game, _) = Game::new(1, 0, 3, &suits, false);

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

}

