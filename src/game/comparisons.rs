use super::{Hand, TrickType};
use crate::cards::{Card, PlayedCard, Rank, Suit};
use std::cmp::Ordering;

pub fn compare_hands(
    last_move: Hand,
    new_hand: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> bool {
    let last_cards = last_move.to_cards();
    let new_cards = new_hand.to_cards();

    if last_cards.len() != new_cards.len() {
        return false;
    }

    match last_move {
        Hand::Single(_) | Hand::Pair(_, _) | Hand::Prial(_, _, _) => {
            let last_card = get_top_card(last_cards, suit_order, rank_order);
            let new_card = get_top_card(new_cards, suit_order, rank_order);
            compare_single(last_card, new_card, suit_order, rank_order) == Ordering::Greater
        }
        Hand::FiveCardTrick(_) => compare_five_cards(last_move, new_hand, suit_order, rank_order),
        _ => false,
    }
}

pub fn sort_played_cards(
    hand: &Vec<PlayedCard>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13]
) -> Vec<PlayedCard> {
    let mut sortable_cards = hand.clone();
    sortable_cards.sort_by(
        |&a, &b| compare_single(a, b, suit_order, rank_order)
    );
    sortable_cards
}

pub fn sort_unplayed_cards(
    hand: &Vec<Card>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13]
) -> Vec<Card> {
    let mut sortable_cards = hand.clone();
    sortable_cards.sort_by(
        |&a, &b| compare_single_unplayed(a, b, suit_order, rank_order)
    );
    sortable_cards
}

fn compare_single(
    last_card: PlayedCard,
    new_card: PlayedCard,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> Ordering {
    let rank_comparison = compare_rank(last_card, new_card, rank_order);

    match rank_comparison {
        Ordering::Equal => compare_suits(last_card, new_card, suit_order),
        x => x,
    }
}

fn compare_single_unplayed(
    last_card: Card,
    new_card: Card,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> Ordering {
    let rank_comparison = compare_rank_unplayed(last_card, new_card, rank_order);

    match rank_comparison {
        Ordering::Equal => compare_suits_unplayed(last_card, new_card, suit_order),
        x => x,
    }

}

pub fn compare_five_cards(
    last_move: Hand,
    new_hand: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> bool {
    let last_trick = match last_move {
        Hand::FiveCardTrick(x) => x,
        _ => panic!("unable to get trick"),
    };
    let new_trick = match new_hand {
        Hand::FiveCardTrick(x) => x,
        _ => panic!("unable to get trick"),
    };

    if new_trick.trick_type > last_trick.trick_type {
        return true;
    }

    if new_trick.trick_type < last_trick.trick_type {
        return false;
    }

    let last_cards = last_move.to_cards();
    let new_cards = new_hand.to_cards();

    let (last_card, new_card) = match last_trick.trick_type {
        TrickType::Straight
        | TrickType::Flush
        | TrickType::FiveOfAKind
        | TrickType::StraightFlush => {
            let last_card = get_top_card(last_cards, suit_order, rank_order);
            let new_card = get_top_card(new_cards, suit_order, rank_order);

            (last_card, new_card)
        }
        TrickType::FullHouse | TrickType::FourOfAKind => {
            let set_count = if last_trick.trick_type == TrickType::FullHouse {
                3
            } else {
                4
            };

            let last_card = get_top_of_n(last_cards, set_count, suit_order, rank_order);

            let new_card = get_top_of_n(new_cards, set_count, suit_order, rank_order);

            (last_card, new_card)
        }
    };

    compare_single(last_card, new_card, suit_order, rank_order) == Ordering::Greater
}

fn get_top_card(
    cards: Vec<PlayedCard>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> PlayedCard {
    *sort_played_cards(
        &cards,
        suit_order,
        rank_order
    ).first().expect("no cards found")
}

fn get_top_of_n(
    cards: Vec<PlayedCard>,
    n: usize,
    suits_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> PlayedCard {
    let counts = Hand::get_counts(cards.clone());
    let mut top_rank = *rank_order.first().unwrap();

    for (rank, count) in &counts {
        if *count == n {
            top_rank = *rank;
        }
    }

    let valid_cards: Vec<PlayedCard> = cards
        .iter()
        .filter(|&c| c.get_rank() == top_rank)
        .map(|&c| c.clone())
        .collect();

    get_top_card(valid_cards, suits_order, rank_order)
}

fn compare_suits(card1: PlayedCard, card2: PlayedCard, suit_order: [Suit; 4]) -> Ordering {
    let c1_i = get_suit_index(card1, suit_order);
    let c2_i = get_suit_index(card2, suit_order);

    c2_i.cmp(&c1_i)
}

fn compare_rank(card1: PlayedCard, card2: PlayedCard, rank_order: [Rank; 13]) -> Ordering {
    let c1_i = get_rank_index(card1, rank_order);
    let c2_i = get_rank_index(card2, rank_order);

    c2_i.cmp(&c1_i)
}

fn get_suit_index(card: PlayedCard, suit_order: [Suit; 4]) -> Option<usize> {
    suit_order.iter().position(|&x| x == card.get_suit())
}

fn get_rank_index(card: PlayedCard, rank_order: [Rank; 13]) -> Option<usize> {
    rank_order.iter().position(|&x| x == card.get_rank())
}

fn compare_suits_unplayed(card1: Card, card2: Card, suit_order: [Suit; 4]) -> Ordering {
    let c1_i = get_suit_index_unplayed(card1, suit_order);
    let c2_i = get_suit_index_unplayed(card2, suit_order);

    c2_i.cmp(&c1_i)
}

fn compare_rank_unplayed(card1: Card, card2: Card, rank_order: [Rank; 13]) -> Ordering {
    let mut c1_i:u8 = get_rank_index_unplayed(card1, rank_order)
        .expect("unable to find rank index") as u8;
    let mut c2_i:u8 = get_rank_index_unplayed(card2, rank_order)
        .expect("unable to find rank index") as u8;

    match card1 {
        Card::Joker{ deck_id: _ } => c1_i += 1,
        _ => (),
    }

    match card2 {
        Card::Joker{ deck_id: _ } => c2_i += 1,
        _ => (),
    }

    c2_i.cmp(&c1_i)
}

fn get_suit_index_unplayed(card: Card, suit_order: [Suit; 4]) -> Option<usize> {
    suit_order.iter().position(|&x| 
        x == card.get_suit().unwrap_or(suit_order[3])
    )
}

fn get_rank_index_unplayed(card: Card, rank_order: [Rank; 13]) -> Option<usize> {
    rank_order.iter().position(|&x|
        x == card.get_rank().unwrap_or(rank_order[12])
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{PlayedCard, Rank, Suit};
    use crate::game::hands::*;

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
    fn it_can_compare_singles() {
        let hand1 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Clubs, false));
        let hand2 = Hand::Single(PlayedCard::new(Rank::Four, Suit::Clubs, false));

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn it_returns_false_when_singles_are_equal() {
        let hand1 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Clubs, false));
        let hand2 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Clubs, false));

        assert!(!compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn it_returns_false_when_second_hand_is_lower_than_first() {
        let hand1 = Hand::Single(PlayedCard::new(Rank::Five, Suit::Clubs, false));
        let hand2 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Clubs, false));

        assert!(!compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn where_ranks_are_equal_it_compares_on_suit() {
        let hand1 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Clubs, false));
        let hand2 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Hearts, false));

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn rank_takes_precendence_over_suit() {
        let hand1 = Hand::Single(PlayedCard::new(Rank::Three, Suit::Spades, false));
        let hand2 = Hand::Single(PlayedCard::new(Rank::Seven, Suit::Clubs, false));

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn in_a_pair_the_highest_card_wins() {
        let hand1 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
        );
        let hand2 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn in_a_pair_the_highest_card_must_be_greater() {
        let hand1 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
        );
        let hand2 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );

        assert!(!compare_hands(
            hand2,
            hand1,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn pair_cannot_be_beaten_by_an_equal_hand() {
        let hand1 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );
        let hand2 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );

        assert!(!compare_hands(
            hand2,
            hand1,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn in_a_prial_the_highest_card_wins() {
        let hand1 = Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
        );
        let hand2 = Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn in_a_prial_the_highest_card_must_be_greater() {
        let hand1 = Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
        );
        let hand2 = Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );

        assert!(!compare_hands(
            hand2,
            hand1,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn prial_cannot_be_beaten_by_an_equal_hand() {
        let hand1 = Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Diamonds, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );
        let hand2 = Hand::Prial(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false),
        );

        assert!(!compare_hands(
            hand2,
            hand1,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn flush_beats_straight() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Seven, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Diamonds, false),
            PlayedCard::new(Rank::Nine, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(Straight, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Clubs, false),
            PlayedCard::new(Rank::Nine, Suit::Clubs, false),
        ];
        let hand2 = build_fct!(Flush, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));

        assert!(!compare_hands(
            hand2,
            hand1,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn higher_straight_beats_lower_straight() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Seven, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Diamonds, false),
            PlayedCard::new(Rank::Nine, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(Straight, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Seven, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Diamonds, false),
            PlayedCard::new(Rank::Nine, Suit::Spades, false),
        ];
        let hand2 = build_fct!(Straight, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn higher_flush_beats_a_lower_flush() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Clubs, false),
            PlayedCard::new(Rank::Nine, Suit::Clubs, false),
        ];
        let hand1 = build_fct!(Flush, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Clubs, false),
            PlayedCard::new(Rank::Ten, Suit::Clubs, false),
        ];
        let hand2 = build_fct!(Flush, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn full_house_beats_a_flush() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Clubs, false),
            PlayedCard::new(Rank::Nine, Suit::Clubs, false),
        ];
        let hand1 = build_fct!(Flush, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Two, Suit::Clubs, false),
            PlayedCard::new(Rank::Two, Suit::Diamonds, false),
        ];
        let hand2 = build_fct!(FullHouse, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn full_house_is_resolved_on_highest_of_3_cards() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Four, Suit::Diamonds, false),
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(FullHouse, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Two, Suit::Clubs, false),
            PlayedCard::new(Rank::Two, Suit::Diamonds, false),
        ];
        let hand2 = build_fct!(FullHouse, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn full_house_is_resolved_on_highest_of_3_cards_suit_if_rank_equal() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Five, Suit::Diamonds, false),
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(FullHouse, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Two, Suit::Clubs, false),
            PlayedCard::new(Rank::Two, Suit::Diamonds, false),
        ];
        let hand2 = build_fct!(FullHouse, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn four_of_a_kind_is_resolved_by_highest_of_4_cards() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
            PlayedCard::new(Rank::Four, Suit::Diamonds, false),
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(FourOfAKind, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Two, Suit::Diamonds, false),
        ];
        let hand2 = build_fct!(FourOfAKind, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn four_of_a_kind_is_resolved_by_highest_of_4_cards_suit_if_rank_equal() {
        let hand1_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Five, Suit::Diamonds, false),
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(FourOfAKind, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Two, Suit::Diamonds, false),
        ];
        let hand2 = build_fct!(FourOfAKind, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn five_of_a_kind_is_resolved_by_highest_card() {
        let hand1_cards = [
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
            PlayedCard::new(Rank::Four, Suit::Diamonds, false),
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Hearts, false),
        ];
        let hand1 = build_fct!(FiveOfAKind, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Hearts, false),
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Diamonds, false),
        ];
        let hand2 = build_fct!(FiveOfAKind, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn straight_flush_is_resolved_by_highest_card() {
        let hand1_cards = [
            PlayedCard::new(Rank::Three, Suit::Spades, false),
            PlayedCard::new(Rank::Four, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Six, Suit::Spades, false),
            PlayedCard::new(Rank::Seven, Suit::Spades, false),
        ];
        let hand1 = build_fct!(StraightFlush, hand1_cards).unwrap();

        let hand2_cards = [
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Seven, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Clubs, false),
            PlayedCard::new(Rank::Nine, Suit::Clubs, false),
        ];
        let hand2 = build_fct!(StraightFlush, hand2_cards).unwrap();

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn it_sorts_hands_by_the_default_suit_and_rank() {
        let hand = vec![
            PlayedCard::new(Rank::Two, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Six, Suit::Clubs, false),
            PlayedCard::new(Rank::Two, Suit::Diamonds, false),
            PlayedCard::new(Rank::Seven, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Clubs, false),
            PlayedCard::new(Rank::Eight, Suit::Clubs, false),
            PlayedCard::new(Rank::Nine, Suit::Clubs, false),
        ];

        let sorted_hand = sort_played_cards(
            &hand,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        );

        assert_eq!(
            sorted_hand,
            vec![
                PlayedCard::new(Rank::Two, Suit::Spades, false),
                PlayedCard::new(Rank::Two, Suit::Diamonds, false),
                PlayedCard::new(Rank::Nine, Suit::Clubs, false),
                PlayedCard::new(Rank::Eight, Suit::Clubs, false),
                PlayedCard::new(Rank::Seven, Suit::Clubs, false),
                PlayedCard::new(Rank::Six, Suit::Clubs, false),
                PlayedCard::new(Rank::Five, Suit::Spades, false),
                PlayedCard::new(Rank::Five, Suit::Clubs, false),
            ]
        );
    }

    #[test]
    fn when_not_a_pass_hands_should_have_same_number_of_cards() {
        let hand1_cards = [
            PlayedCard::new(Rank::Three, Suit::Spades, false),
            PlayedCard::new(Rank::Four, Suit::Spades, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Six, Suit::Spades, false),
            PlayedCard::new(Rank::Seven, Suit::Spades, false),
        ];
        let hand1 = build_fct!(StraightFlush, hand1_cards).unwrap();
        let hand2 = Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        );

        assert!(!compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }
}
