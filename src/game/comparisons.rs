use super::{Player, Hand};
use crate::cards::{Suit, Rank, Card, PlayedCard};

#[derive(Debug, PartialEq)]
enum Comparison {
    Greater,
    Equal,
    Lesser,
}

pub fn compare_hand(
    last_move: Hand,
    new_hand: Hand,
    suit_order: [Suit; 4],
    rank_order: [Rank; 13],
) -> bool {
    let card = new_hand.to_cards()[0];

    match last_move {
        Hand::Single(c) => {
            let suit_comparison = compare_suits(c, card, suit_order);
            suit_comparison == Comparison::Greater
                || (
                    suit_comparison == Comparison::Equal 
                    && compare_rank(c, card, rank_order) == Comparison::Greater
                   )
        },
        _ => false
    }
}

fn compare_suits(
    card1: PlayedCard,
    card2: PlayedCard,
    suit_order: [Suit;4],
) -> Comparison {
    let c1_i = get_suit_index(card1, suit_order);
    let c2_i = get_suit_index(card2, suit_order);

    if c1_i == c2_i {
        Comparison::Equal
    } else if c1_i < c2_i {
        Comparison::Greater
    } else {
        Comparison::Lesser
    }
}

fn compare_rank(
    card1: PlayedCard,
    card2: PlayedCard,
    rank_order: [Rank;13],
) -> Comparison {
    let c1_i = get_rank_index(card1, rank_order);
    let c2_i = get_rank_index(card2, rank_order);

    if c1_i == c2_i {
        Comparison::Equal
    } else if c1_i < c2_i {
        Comparison::Greater
    } else {
        Comparison::Lesser
    }
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

