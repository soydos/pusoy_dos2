use crate::cards::{Card, PlayedCard, Rank, Suit};
use std::collections::HashMap;

pub fn find_pairs(hand: &Vec<Card>) -> Vec<Vec<PlayedCard>> {
    get_sets_of_same_rank(2, hand)
}

pub fn find_prials(hand: &Vec<Card>) -> Vec<Vec<PlayedCard>> {
    get_sets_of_same_rank(3, hand)
}

pub fn find_fct(hand: &Vec<Card>) -> Vec<Vec<PlayedCard>> {
    vec![
        vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Four, Suit::Clubs, false),
            PlayedCard::new(Rank::Five, Suit::Spades, false),
            PlayedCard::new(Rank::Six, Suit::Hearts, false),
            PlayedCard::new(Rank::Seven, Suit::Clubs, false),
        ]
    ]
}

pub fn get_sets_of_same_rank(
    n: usize,
    player_hand: &Vec<Card>,
) -> Vec<Vec<PlayedCard>> {
    let counts = get_counts(player_hand.clone());
    let mut rank_options = vec!();
    for (r, count) in &counts {
        if *count == n {
            rank_options.push(*r);
        }
    }

    rank_options.sort();

    let mut hands = vec!();

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

        hands.push(hand.clone());
    }

    hands
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

fn get_natural_cards(hand: &Vec<Card>) -> Vec<Card> {
    hand.iter().filter(|c| {
        c.get_rank() != None
    })
    .map(|&c| c.clone()).collect::<Vec<Card>>()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;
/*
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
*/

    #[test]
    fn it_can_find_pairs_in_a_hand() {
        let hand = vec![
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs} 
        ];

        assert_eq!(find_pairs(&hand).len(), 1);
        assert_eq!(find_pairs(&hand)[0], vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ]);
    }

    #[test]
    fn it_gets_all_the_pairs() {
        let hand = vec![
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs},

        ];

        assert_eq!(find_pairs(&hand).len(), 2);
    }

    #[test]
    fn it_can_find_prials() {
        let hand = vec![
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs} 
        ];

        assert_eq!(find_prials(&hand).len(), 1);
        assert_eq!(find_prials(&hand)[0], vec![
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
            PlayedCard::new(Rank::Three, Suit::Clubs, false),
        ]);
    }

    #[test]
    fn it_can_find_a_straight() {
        let hand = vec![
            Card::Standard{rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{rank: Rank::Five, suit: Suit::Spades},
            Card::Standard{rank: Rank::Six, suit: Suit::Hearts},
            Card::Standard{rank: Rank::Seven, suit: Suit::Clubs},
        ];

        assert_eq!(find_fct(&hand).len(), 1);
    }
}
