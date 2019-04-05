use crate::game::{
    Hand,
    Player,
    compare_hands,
    sort_unplayed_cards
};
use crate::cards::{Card, PlayedCard, Rank, Suit};
use super::{find_pairs, get_sets_of_same_rank, find_fct};

pub fn get_move(
    last_move: Option<Hand>,
    player_option: Option<Player>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> Option<Vec<PlayedCard>> {
    let player = player_option.unwrap();
    // todo - sort
    let unsorted_player_hand = player.get_hand();
    let mut sorted_player_hand = sort_unplayed_cards(
        &unsorted_player_hand,
        suit_order,
        rank_order
    );

    sorted_player_hand.reverse();
    let player_hand = sorted_player_hand;

    if last_move == None {
        return Some(get_all_low_cards(&player_hand))
    } 

    let move_hand = last_move.unwrap();
    match move_hand {
        Hand::Pass => {

            let cards_left = player_hand.len();
            let num_jokers = get_jokers(&player_hand).len();

            if cards_left == num_jokers {
                let hand = match cards_left {
                    1 | 2 | 3 | 5 => player_hand.clone(),
                    _ => vec!(
                            *player_hand.clone().first().unwrap(),
                        )
                };
                return Some(convert_to_played(
                    &hand,
                    suit_order,
                    rank_order
                ));
            }

            let pairs = find_pairs(&player_hand);
            let fct = find_fct(&player_hand);

            let first_pair = if pairs.len() > 0 {
                Some(pairs.first().unwrap().to_vec())
            } else {
                None
            };


            let first_fct = if fct.len() > 0 {
                Some(fct.first().unwrap().to_vec())
            } else {
                None
            };

            let lowest_natural_card = get_lowest_natural_card(
                &player_hand
            );

            if first_fct != None {
                if first_fct.iter().any(|t| {
                    t.iter().any(|&p| {
                        p == lowest_natural_card[0]
                    })
                }) {
                    return first_fct.clone();
                }
            }

            if first_pair != None {
                if first_pair.iter().any(|p| {
                    p[0] == lowest_natural_card[0]
                }) {
                    return first_pair.clone();
                }
            }

            Some(lowest_natural_card)
        },
        Hand::Single(_) => {

            let pairs = find_pairs(&player_hand);

            let single_cards = player_hand.iter().filter(|&p1| {
                !pairs.iter().any(|pair| {
                    pair.iter().any(|p2| *p1 == p2.to_card())
                })
            }).map(|c| c.clone()).collect();

            let played_single = 
                get_lowest_natural_card_against_played(
                    &single_cards,
                    move_hand,
                    suit_order,
                    rank_order
                );

            if played_single != None {
                return played_single;
            }


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

            get_pass()
        },
        Hand::Pair(_, _) | Hand::Prial(_, _, _) => {
            let hand = get_beating_multiple_card_hand(
                move_hand.to_cards().len(),
                &player_hand,
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
        Hand::FiveCardTrick(_) => {
            for trick in find_fct(&player_hand) {
                let built_hand = Hand::build(
                    trick.to_vec()
                ).unwrap();
                if compare_hands(
                    move_hand,
                    built_hand,
                    suit_order,
                    rank_order) {
                    return Some(trick.to_vec());
                }
            }
            get_pass()
        },
    }
    
}

fn get_beating_multiple_card_hand(
    n: usize,
    player_hand: &Vec<Card>,
    move_hand: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13], 
) -> Option<Vec<PlayedCard>> {
    for hand in get_sets_of_same_rank(n, player_hand) {
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

fn get_pass() -> Option<Vec<PlayedCard>>{
    Some(vec!())
}

fn get_all_low_cards(hand: &Vec<Card>) -> Vec<PlayedCard> {
    let natural_cards = get_natural_cards(hand);
    let player_card = natural_cards.first();
    let (_head, tail_cards) = natural_cards.split_at(1);

    if player_card.is_some() {
        let card = player_card.unwrap();
        let mut all_low_cards = vec![
            PlayedCard::new(
                card.get_rank()
                    .unwrap(),
                card.get_suit()
                    .unwrap(),
                false)
        ];

        for c in tail_cards {
            if c.get_rank() == card.get_rank() {
                all_low_cards.push(
                    PlayedCard::new(
                        c.get_rank().unwrap(),
                        c.get_suit().unwrap(),
                        false
                    )
                );
            }
        }

        return all_low_cards;
    }

    vec![]
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

fn convert_to_played(
    hand: &Vec<Card>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13]
) -> Vec<PlayedCard> {
    hand.iter().map(|&c| {
        match c {
            Card::Standard{
                deck_id, rank, suit
            } => {
                PlayedCard::new(rank, suit, false)
            },
            Card::Joker{deck_id} => PlayedCard::new(
                rank_order[0],
                suit_order[0],
                true
            )
        }
    }).collect::<Vec<PlayedCard>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;
    use crate::game::{TrickType, Trick};

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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs}
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs}
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs}
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
            Card::Joker{deck_id: 0},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs}
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Joker{deck_id: 0},
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Spades},
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Spades},
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
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},
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
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Spades},
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
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},

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
    fn ai_can_play_flush() {
         let previous_move = Some(Hand::FiveCardTrick(Trick{
            trick_type: TrickType::Flush,
            cards: [
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Eight, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ]
        }));
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Eight, suit: Suit::Spades},

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
                    Rank::Six, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Six, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Seven, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Seven, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Eight, Suit::Spades, false
                ),
            ))
        );
    }

    #[test]
    fn ai_can_play_flush_when_it_has_more_than_five_of_a_suit() {
         let previous_move = Some(Hand::FiveCardTrick(Trick{
            trick_type: TrickType::Flush,
            cards: [
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Eight, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ]
        }));
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Eight, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Eight, suit: Suit::Spades},

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
                    Rank::Six, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Six, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Seven, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Seven, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Eight, Suit::Spades, false
                ),
            ))
        );
    }


    #[test]
    fn ai_can_pass_on_fct() {
         let previous_move = Some(Hand::FiveCardTrick(Trick{
            trick_type: TrickType::Flush,
            cards: [
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Eight, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ]
        }));
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},

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
    fn if_ai_has_a_joker_left_on_an_empty_table_it_will_play() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Joker{deck_id: 0}
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
                PlayedCard::new(Rank::Three, Suit::Clubs, true)
            ))
        );
    }

    #[test]
    fn if_ai_only_has_jokers_left_it_will_play_them() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Joker{deck_id: 0},
            Card::Joker{deck_id: 0},
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
                PlayedCard::new(Rank::Three, Suit::Clubs, true),
                PlayedCard::new(Rank::Three, Suit::Clubs, true)
            ))
        );
    }

    #[test]
    fn if_ai_only_has_4_jokers_left_it_will_play_one() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Joker{deck_id: 0},
            Card::Joker{deck_id: 0},
            Card::Joker{deck_id: 0},
            Card::Joker{deck_id: 0},
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
                PlayedCard::new(Rank::Three, Suit::Clubs, true),
            ))
        );
    }

    #[test]
    fn ai_will_lead_with_low_pair_if_possible() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Ace, suit: Suit::Clubs},
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
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ))
        );
    }

    #[test]
    fn ai_will_not_lead_with_pair_if_lower_single() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Ace, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Ace, suit: Suit::Clubs},
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
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ))
        );
    }

    #[test]
    fn ai_will_lead_with_pair_if_possible() {
        let previous_move = Some(Hand::Pass);
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Queen, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::King, suit: Suit::Clubs},
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
                PlayedCard::new(Rank::Jack, Suit::Clubs, false),
                PlayedCard::new(Rank::Jack, Suit::Hearts, false),
            ))
        );
    }

    #[test]
    fn ai_wont_split_pair_for_single() {
        let previous_move = Some(Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ));

        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Queen, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::King, suit: Suit::Clubs},
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
                PlayedCard::new(Rank::Queen, Suit::Clubs, false),
            ))
        );
    }

    #[test]
    fn ai_could_open_on_a_pair() {
        let previous_move = None;
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Queen, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::King, suit: Suit::Clubs},
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
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Spades, false),
            ))
        );
    }

    #[test]
    fn it_can_beat_a_straight_with_another_straight() {
         let previous_move = Some(Hand::FiveCardTrick(Trick{
            trick_type: TrickType::Straight,
            cards: [
                PlayedCard::new(Rank::Seven, Suit::Spades, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Five, Suit::Hearts, false),
                PlayedCard::new(Rank::Four, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ]
        }));
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Eight, suit: Suit::Spades},

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
                    Rank::Four, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Five, Suit::Clubs, false
                ),
                PlayedCard::new(
                    Rank::Six, Suit::Spades, false
                ),
                PlayedCard::new(
                    Rank::Seven, Suit::Clubs, false
                ),
                PlayedCard::new(
                    Rank::Eight, Suit::Spades, false
                ),
            ))
        );
    }


    #[test]
    fn it_passes_on_a_straight_when_it_cant_play() {
         let previous_move = Some(Hand::FiveCardTrick(Trick{
            trick_type: TrickType::Straight,
            cards: [
                PlayedCard::new(Rank::Seven, Suit::Spades, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Five, Suit::Hearts, false),
                PlayedCard::new(Rank::Four, Suit::Clubs, false),
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
            ]
        }));
        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Ten, suit: Suit::Spades},

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
    fn it_splits_a_pair_when_it_has_no_other_option() {
        let previous_move = Some(Hand::Single(
            PlayedCard::new(Rank::Queen, Suit::Clubs, false)
        ));

        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Jack, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Queen, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Queen, suit: Suit::Spades},
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
                PlayedCard::new(Rank::Queen, Suit::Spades, false),
            ))
        );
    }

    #[test]
    fn it_can_open_with_a_straight() {
        let previous_move = Some(Hand::Pass);

        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Spades},
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
                PlayedCard::new(Rank::Three, Suit::Clubs, false),
                PlayedCard::new(Rank::Four, Suit::Hearts, false),
                PlayedCard::new(Rank::Five, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Spades, false),
                PlayedCard::new(Rank::Seven, Suit::Spades, false),
            ))
        );
    }

    #[test]
    fn it_plays_a_joker_when_it_has_no_other_choice() {
        let previous_move = Some(Hand::Single(
            PlayedCard::new(Rank::Ace, Suit::Spades, false)
        ));

        let hand = vec!(
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Spades},
            Card::Joker{deck_id:0}
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
                PlayedCard::new(Rank::Two, Suit::Spades, true),
            ))
        );
    }

    #[test]
    fn it_respects_alternative_suit_rank_orders() {
        let alternative_suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades
        ];

        let alternative_rank_order = [
            Rank::Two,
            Rank::Ace,
            Rank::King,
            Rank::Queen,
            Rank::Jack,
            Rank::Ten,
            Rank::Nine,
            Rank::Eight,
            Rank::Seven,
            Rank::Six,
            Rank::Five,
            Rank::Four,
            Rank::Three,
        ];

        let previous_move = Some(Hand::Pass);

        let hand = vec!(
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Five, suit: Suit::Clubs},
        );
        let player = Player::new("cpu".to_string(), hand);

        assert_eq!(
            get_move(
                previous_move,
                Some(player),
                alternative_suit_order,
                alternative_rank_order,
            ),
            Some(vec!(
                PlayedCard::new(Rank::Five, Suit::Clubs, false),
            ))
        );

    }
}
