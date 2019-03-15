use crate::game::{Hand, Player, compare_hands};
use crate::cards::{Card, PlayedCard, Rank, Suit};
use std::collections::HashMap;

pub fn get_move(
    last_move: Option<Hand>,
    player_option: Option<Player>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> Option<Vec<PlayedCard>> {

    let player = player_option.unwrap();
    // todo - sort
    let player_hand = player.get_hand();

    if last_move == None {
        return Some(get_lowest_natural_card(&player_hand))
    } 

    let move_hand = last_move.unwrap();
    match move_hand {
        Hand::Pass => Some(get_lowest_natural_card(&player_hand)),
        Hand::Single(_) => {
            let played_single = 
                get_lowest_natural_card_against_played(
                    &player_hand,
                    move_hand,
                    suit_order,
                    rank_order
                );
            if played_single != None {
                return played_single;
            }

            if player_hand.len() < 3 {
                let jokers = get_jokers(&player_hand);

                if jokers.len() > 0 {
                    let player_hand = get_winning_joker(
                        suit_order,
                        rank_order,
                        move_hand,
                    );
                    if player_hand != None {
                        return player_hand;
                    }
                }
            }

            get_pass()
        },
        Hand::Pair(_, _) | Hand::Prial(_, _, _) => {
            let hand = get_multiple_card_hand(
                move_hand.to_cards().len(),
                player_hand,
                move_hand,
                suit_order,
                rank_order,
            );

            if hand.is_none() {
                get_pass()
            } else {
                hand
            }

        },
        _ => get_pass()
    }
    
}

fn get_multiple_card_hand(
    n: usize,
    player_hand: Vec<Card>,
    move_hand: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13], 
) -> Option<Vec<PlayedCard>> {
    let counts = get_counts(player_hand.clone());
    let mut rank_options = vec!();
    for (r, count) in &counts {
        if *count == n {
            rank_options.push(*r);
        }
    }

    rank_options.sort();

    for &rank in &rank_options {
        let mut hand = vec!();
        for card in get_natural_cards(&player_hand) {
            if card.get_rank().unwrap() == rank {
                hand.push(
                    PlayedCard::new(
                        rank,
                        card.get_suit().unwrap(),
                        false
                    )
                );
            }
        }

        let built_hand = Hand::build(hand.clone()).unwrap();
        if compare_hands(
            move_hand,
            built_hand,
            suit_order,
            rank_order) {
            return Some(hand.clone());
        }
    }

    None
}

fn get_counts(cards: Vec<Card>) -> HashMap<Rank, usize> {
        cards.iter()
            .filter(|c| !c.get_rank().is_none())
            .fold(HashMap::new(), |mut acc, &card| {
                *acc.entry(
                    card.get_rank().unwrap()
                ).or_insert(0) += 1;
                acc
            })
    }

fn get_pass() -> Option<Vec<PlayedCard>>{
    Some(vec!())
}

fn get_lowest_natural_card(hand: &Vec<Card>) -> Vec<PlayedCard>{
    let natural_cards = get_natural_cards(hand);
    let player_card = natural_cards.first();
    match player_card {
        Some(card)  =>  vec!(
            PlayedCard::new(
                card.get_rank()
                    .unwrap(),
                card.get_suit()
                    .unwrap(),
                false)
        ),
        None        => vec!()
    }
}

fn get_lowest_natural_card_against_played(
    hand: &Vec<Card>,
    last_move: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13]
) -> Option<Vec<PlayedCard>> {
    let natural_cards = get_natural_cards(hand);
    for player_card in natural_cards {
        let player_hand = Hand::build(
            vec!(PlayedCard::new(
                player_card.get_rank()
                    .unwrap(),
                player_card.get_suit()
                    .unwrap(),
                false
            ))
        ).unwrap();
        if compare_hands(
            last_move,
            player_hand, 
            suit_order,
            rank_order
        ) {
            return Some(player_hand.to_cards());
        }
    }
    None
}

fn get_winning_joker(
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
    last_move: Hand,
) -> Option<Vec<PlayedCard>> {
    let joker_single = Hand::build(
        vec!(PlayedCard::new(
            *rank_order.last().unwrap(),
            *suit_order.last().unwrap(),
            true
        ))
    ).unwrap();

    match compare_hands(
        last_move,
        joker_single, 
        suit_order,
        rank_order
    ) {
        true => Some(joker_single.to_cards()),
        false => None
    }

}

fn get_natural_cards(hand: &Vec<Card>) -> Vec<Card> {
    hand.iter().filter(|c| {
        c.get_rank() != None
    })
    .map(|&c| c.clone()).collect::<Vec<Card>>()
}

fn get_jokers(hand: &Vec<Card>) -> Vec<Card>{
    hand.iter().filter(|c| {
        c.get_rank() == None
    })
    .map(|&c| c.clone()).collect::<Vec<Card>>()

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

    #[test]
    fn ai_plays_a_joker_to_go_for_win() {
         let previous_move = Some(Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Joker,
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
                    Rank::Two, Suit::Spades, true
                )
            ))
        );
    }

    #[test]
    fn ai_cant_play_a_joker_if_it_doesnt_have_one() {
         let previous_move = Some(Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!())
        );
    }


    #[test]
    fn ai_can_play_a_pair() {
         let previous_move = Some(Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Three, suit: Suit::Spades},
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
                ),
                PlayedCard::new(
                    Rank::Three, Suit::Spades, false
                ),
            ))
        );
    }

    #[test]
    fn ai_passes_on_pair_when_it_cant_play_a_pair() {
         let previous_move = Some(Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Spades},
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!())
        );
    }

    #[test]
    fn ai_plays_higher_pair_where_it_can() {
         let previous_move = Some(Hand::Pair(
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{rank: Rank::Six, suit: Suit::Clubs},
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
                    Rank::Six, Suit::Clubs, false
                ),
                PlayedCard::new(
                    Rank::Six, Suit::Spades, false
                ),
            ))
        );
    }

    #[test]
    fn ai_plays_a_prial_where_it_can() {
         let previous_move = Some(Hand::Prial(
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{rank: Rank::Six, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Six, suit: Suit::Clubs},
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
                    Rank::Six, Suit::Clubs, false
                ),
                PlayedCard::new(
                    Rank::Six, Suit::Clubs, false
                ),

                PlayedCard::new(
                    Rank::Six, Suit::Spades, false
                ),
            ))
        );
    }

    #[test]
    fn ai_passes_on_prial_when_it_cant_play_beat_the_prial() {
         let previous_move = Some(Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Spades},
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                DEFAULT_SUIT_ORDER,
                DEFAULT_RANK_ORDER,
            ),
            Some(vec!())
        );
    }

    #[test]
    fn ai_plays_the_lowest_prial_it_can() {
         let previous_move = Some(Hand::Prial(
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
        ));
        let hand = vec!(
            Card::Standard{rank: Rank::Seven, suit: Suit::Spades},
            Card::Standard{rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{rank: Rank::Six, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Six, suit: Suit::Clubs},

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
                    Rank::Six, Suit::Clubs, false
                ),
                PlayedCard::new(
                    Rank::Six, Suit::Clubs, false
                ),

                PlayedCard::new(
                    Rank::Six, Suit::Spades, false
                ),
            ))
        );

    }

}
