use crate::game::{Hand, Player};
use crate::cards::{PlayedCard, Rank, Suit};

pub fn get_move(
    last_move: Option<Hand>,
    player: Option<Player>,
) -> Option<Vec<PlayedCard>> {

    if last_move == None {
        Some(
            vec!(
                PlayedCard::new(
                    Rank::Three, Suit::Clubs, false
                )
            )
        )
    } else {
        Some(vec!())
    }
}
