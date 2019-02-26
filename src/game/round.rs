use super::Player;
use crate::cards::{Suit, Rank, Card};

#[derive(Clone, Debug)]
pub struct Round{
    pub players: Vec<Player>,
    pub next_player: Option<&'static str>,
}

impl Round {
    pub fn new(
        players: Vec<Player>,
        next_player: Option<&'static str>,
    ) -> Round {
        Round{
            players,
            next_player,
        }
    }

    pub fn get_next_player(&self) -> Option<&str> {
        match self.next_player {
            None => self.get_starting_player(),
            x    => x
        }
    }

    pub fn get_starting_player(&self) -> Option<&str> {
        let three_clubs = Card::Standard{
            suit: Suit::Clubs,
            rank: Rank::Three,
        };
        for player in self.players.iter() {
            if player.get_hand().contains(&three_clubs) {
                return Some(player.get_id());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn when_game_hasnt_started_player_with_3clubs_starts() {
        let a_cards = vec![
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs}
        ];
        let b_cards = vec![
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs}
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);

        let players = vec![player_a, player_b];

        let round = Round::new(players, None);

        assert_eq!(round.get_next_player(), Some("a"));
    }


    #[test]
    fn when_game_has_started_there_will_be_a_current_player() {
        let a_cards = vec![
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs}
        ];
        let b_cards = vec![
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs}
        ];
        let player_a = Player::new("a".to_string(), a_cards);
        let player_b = Player::new("b".to_string(), b_cards);

        let players = vec![player_a, player_b];

        let round = Round::new(players, Some("b"));

        assert_eq!(round.get_next_player(), Some("b"));

    }
}

