use crate::game::{Hand, Player, compare_hands};
use crate::cards::{PlayedCard, Rank, Suit};

pub fn get_move(
    last_move: Option<Hand>,
    player: Option<Player>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> Option<Vec<PlayedCard>> {

    if last_move == None {
        Some(vec!(
            PlayedCard::new(rank_order[0], suit_order[0], false)
        ))
    } else {
        let move_hand = last_move.unwrap();
        match move_hand {
            Hand::Pass => {
                for card in player.unwrap().get_hand() {
                    if card.get_rank() != None {
                        return Some(vec!(
                            PlayedCard::new(
                                card.get_rank().unwrap(),
                                card.get_suit().unwrap(),
                                false
                            )
                        ));
                    }
                }
                return Some(vec!());
            },
            Hand::Single(_) => {
                for player_card in player.unwrap().get_hand() {
                    let player_hand = Hand::build(
                        vec!(PlayedCard::new(
                            player_card.get_rank()
                                .unwrap_or(Rank::Three),
                            player_card.get_suit()
                                .unwrap_or(Suit::Clubs),
                            false
                        ))
                    ).unwrap();
                    if compare_hands(
                        move_hand,
                        player_hand, 
                        suit_order,
                        rank_order
                    ) {
                        return Some(player_hand.to_cards());
                    }
                }
                Some(vec!())
            },
            _ => Some(vec!())
        }
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
    fn ai_plays_lowest_card_at_start_of_game() {
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs}
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                None,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!(
                PlayedCard::new(
                    Rank::Three, Suit::Clubs, false
                )
            ))
        );
    }

    #[test]
    fn ai_plays_the_lowest_single_it_can() {
        let previous_move = Some(Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs}
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!(
                PlayedCard::new(
                    Rank::Four, Suit::Clubs, false
                )
            ))
        );
    }

    #[test]
    fn ai_plays_lowest_single_on_a_pass() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs}
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!(
                PlayedCard::new(
                    Rank::Three, Suit::Clubs, false
                )
            ))
        );
    }

    #[test]
    fn ai_plays_lowest_real_single_on_a_pass() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Joker,
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs}
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!(
                PlayedCard::new(
                    Rank::Three, Suit::Clubs, false
                )
            ))
        );
    }

}
