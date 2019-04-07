use crate::cards::{Card, PlayedCard, Rank, Suit};
use std::collections::HashMap;

pub fn find_pairs(hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    get_sets_of_same_rank(2, hand)
}

pub fn find_prials(hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    get_sets_of_same_rank(3, hand)
}

pub fn find_fct(hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    let natural_cards = get_natural_cards(hand.to_vec());
    let straights = get_straights(&natural_cards);
    let flushes = get_flushes(&natural_cards);
    let full_houses = get_full_houses(&natural_cards);
    let four_of_a_kinds = get_four_of_a_kinds(&natural_cards);
    let mut five_card_tricks = vec![];

    five_card_tricks.extend(straights);
    five_card_tricks.extend(flushes);
    five_card_tricks.extend(full_houses);
    five_card_tricks.extend(four_of_a_kinds);

    five_card_tricks
}

fn get_straights(hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    let mut straights = vec![];
    for card in hand {
        let mut sequence = vec![
            PlayedCard::new(
                card.get_rank().unwrap(),
                card.get_suit().unwrap(),
                false
            )
        ];
        let mut next_rank = get_next_rank(*card);
        for c2 in hand {
            if c2.get_rank() == next_rank {
                sequence.push(PlayedCard::new(
                    c2.get_rank().unwrap(),
                    c2.get_suit().unwrap(),
                    false
                ));
                next_rank = get_next_rank(*c2);
            }
            if sequence.len() == 5 {
                break;
            }
        }

        if sequence.len() == 5 {
            straights.push(sequence);
        }
    }
    straights
}

fn get_flushes(hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    let mut flushes = vec![];
    let counts = get_suit_counts(hand);

    for (r, count) in &counts {
        if *count >= 5 {
            let flush_suit = Some(*r);
            let mut flush = vec!();
            for card in hand {
                if card.get_suit() == flush_suit {
                    flush.push(
                        PlayedCard::new(
                            card.get_rank().unwrap(),
                            flush_suit.unwrap(),
                            false
                        )
                    );
                }
                if flush.len() == 5 {
                    break;
                }
            }

            if flush.len() == 5 {
                flushes.push(flush);
            }
        }
    }
    flushes
}

fn get_full_houses(hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    let mut full_houses = vec![];
    let pairs = find_pairs(&hand.to_vec());
    let prials = find_prials(&hand.to_vec());
   
    for prial in &prials {
        for pair in &pairs {
            let mut full_house = prial.clone();
            full_house.extend(pair);
            full_houses.push(full_house);
        }
    }

    full_houses
}

pub fn get_four_of_a_kinds(
    hand: &[Card]) -> Vec<Vec<PlayedCard>> {
    let mut four_of_a_kinds = vec![];
    let fours = get_sets_of_same_rank(4, hand);

    for four in &fours {
        let mut four_of_a_kind = four.clone();
        for c in hand {
            let played_card = PlayedCard::new(
                c.get_rank().unwrap(),
                c.get_suit().unwrap(),
                false);

            if !four.contains(&played_card) {
                four_of_a_kind.push(played_card);
                break;
            }
        }

        if four_of_a_kind.len() == 5 {
            four_of_a_kinds.push(four_of_a_kind);
        }
    }

    four_of_a_kinds
}

pub fn get_sets_of_same_rank(
    n: usize,
    player_hand: &[Card],
) -> Vec<Vec<PlayedCard>> {
    let counts = get_counts(player_hand.to_owned());
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
        for card in get_natural_cards(player_hand.to_vec()) {
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
        .filter(|c| c.get_rank().is_some())
        .fold(HashMap::new(), |mut acc, &card| {
            *acc.entry(
                card.get_rank().unwrap()
            ).or_insert(0) += 1;
            acc
        })
}

fn get_suit_counts(cards: &[Card]) -> HashMap<Suit, usize> {
    cards.iter()
        .filter(|c| c.get_rank().is_some())
        .fold(HashMap::new(), |mut acc, &card| {
            *acc.entry(
                card.get_suit().unwrap()
            ).or_insert(0) += 1;
            acc
        })
}

fn get_natural_cards(hand: Vec<Card>) -> Vec<Card> {
    hand.iter().filter(|c| {
        c.get_rank() != None
    })
    .cloned().collect::<Vec<Card>>()
}

fn get_next_rank(card: Card) -> Option<Rank> {
    match card.get_rank().unwrap_or(Rank::Two) {
        Rank::Three => Some(Rank::Four),
        Rank::Four => Some(Rank::Five),
        Rank::Five => Some(Rank::Six),
        Rank::Six => Some(Rank::Seven),
        Rank::Seven => Some(Rank::Eight),
        Rank::Eight => Some(Rank::Nine),
        Rank::Nine => Some(Rank::Ten),
        Rank::Ten => Some(Rank::Jack),
        Rank::Jack => Some(Rank::Queen),
        Rank::Queen => Some(Rank::King),
        Rank::King => Some(Rank::Ace),
        Rank::Ace => Some(Rank::Two),
        Rank::Two => None
    }
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs} 
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs},

        ];

        assert_eq!(find_pairs(&hand).len(), 2);
    }

    #[test]
    fn it_can_find_prials() {
        let hand = vec![
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs} 
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
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Seven, suit: Suit::Clubs},
        ];

        assert_eq!(find_fct(&hand).len(), 1);
    }

    #[test]
    fn it_returns_an_empty_vector_when_there_is_no_tricks() {
        let hand = vec![
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Ten, suit: Suit::Clubs},
        ];

        assert_eq!(find_fct(&hand).len(), 0);
    }

    #[test]
    fn it_can_find_a_flush() {
        let hand = vec![
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Five, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Six, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Ten, suit: Suit::Clubs},
        ];

        assert_eq!(find_fct(&hand).len(), 1);
    }

    #[test]
    fn it_can_find_full_houses() {
        let hand = vec![
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Diamonds},
        ];

        assert_eq!(find_fct(&hand).len(), 1);
    }

    #[test]
    fn it_can_find_four_of_a_kind() {
        let hand = vec![
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Spades},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Hearts},
            Card::Standard{deck_id: 0, rank: Rank::Three, suit: Suit::Clubs},
            Card::Standard{deck_id: 0, rank: Rank::Four, suit: Suit::Diamonds},
        ];
        assert_eq!(find_fct(&hand).len(), 1);
    }
}
