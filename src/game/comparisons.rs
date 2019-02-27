use std::cmp::Ordering;
use super::Hand;
use crate::cards::{Suit, Rank, PlayedCard};

pub fn compare_hands(
    last_move: Hand,
    new_hand: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> bool {
    let last_cards = last_move.to_cards();
    let new_cards = new_hand.to_cards();

    match last_move {
        Hand::Single(_) => {
            let last_card = last_cards[0];
            let new_card = new_cards[0];
            compare_single(
                last_card,
                new_card,
                suit_order,
                rank_order
            ) == Ordering::Greater
        },
        Hand::Pair(_, _) | Hand::Prial(_, _, _) => {
            let last_card = get_top_card(
                last_cards,
                suit_order,
                rank_order
            );
            let new_card = get_top_card(
                new_cards,
                suit_order,
                rank_order
            );
            compare_single(
                last_card,
                new_card,
                suit_order,
                rank_order
            ) == Ordering::Greater
        },
        _ => false
    }
}

fn compare_single(
    last_card: PlayedCard,
    new_card: PlayedCard,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> Ordering {
    let rank_comparison = compare_rank(
        last_card,
        new_card,
        rank_order
    );

    match rank_comparison {
        Ordering::Equal => {
            compare_suits(
                last_card,
                new_card,
                suit_order
            )
        },
        x  => x
    }
}

fn get_top_card(
    cards: Vec<PlayedCard>,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> PlayedCard {
    let mut sortable_cards = cards.clone();
    sortable_cards.sort_by(|&a, &b| {
        compare_single(a, b, suit_order, rank_order)
    });

    sortable_cards[0]
}


fn compare_suits(
    card1: PlayedCard,
    card2: PlayedCard,
    suit_order: [Suit;4],
) -> Ordering {
    let c1_i = get_suit_index(card1, suit_order);
    let c2_i = get_suit_index(card2, suit_order);

    c2_i.cmp(&c1_i)
}

fn compare_rank(
    card1: PlayedCard,
    card2: PlayedCard,
    rank_order: [Rank;13],
) -> Ordering {
    let c1_i = get_rank_index(card1, rank_order);
    let c2_i = get_rank_index(card2, rank_order);

    c2_i.cmp(&c1_i)
}

fn get_suit_index(
    card:PlayedCard,
    suit_order:[Suit;4]
) -> Option<usize> {
    suit_order.iter().position(|&x| x == card.get_suit())
}

fn get_rank_index(
    card:PlayedCard,
    rank_order:[Rank;13]
) -> Option<usize> {
    rank_order.iter().position(|&x| x == card.get_rank())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Rank, Suit, PlayedCard};

    static DEFAULT_SUIT_ORDER: [Suit;4] = [
        Suit::Clubs,
        Suit::Hearts,
        Suit::Diamonds,
        Suit::Spades, 
    ];

    static DEFAULT_RANK_ORDER: [Rank;13] = [
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
        let hand1 = Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ); 
        let hand2 = Hand::Single(
            PlayedCard::new(Rank::Four, Suit::Clubs, false)
        );

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn it_returns_false_when_singles_are_equal() {
        let hand1 = Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ); 
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

    #[test]
    fn it_returns_false_when_second_hand_is_lower_than_first() {
        let hand1 = Hand::Single(
            PlayedCard::new(Rank::Five, Suit::Clubs, false)
        ); 
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

    #[test]
    fn where_ranks_are_equal_it_compares_on_suit() {
        let hand1 = Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Clubs, false)
        ); 
        let hand2 = Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Hearts, false)
        );

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));
    }

    #[test]
    fn rank_takes_precendence_over_suit() {
        let hand1 = Hand::Single(
            PlayedCard::new(Rank::Three, Suit::Spades, false)
        ); 
        let hand2 = Hand::Single(
            PlayedCard::new(Rank::Seven, Suit::Clubs, false)
        );

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
            PlayedCard::new(Rank::Three, Suit::Diamonds, false)
        ); 
        let hand2 = Hand::Pair(
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Spades, false)
        );

        assert!(compare_hands(
            hand1,
            hand2,
            DEFAULT_SUIT_ORDER,
            DEFAULT_RANK_ORDER,
        ));

    }
}
