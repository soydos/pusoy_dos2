use crate::cards::{SuitContext, Deck};
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
            .map(|c| Player::new(c.clone()))
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

}

