use nanoid;
use crate::cards::HandCard;


pub struct Player<'a> {
    id: String,
    hand: Vec<HandCard<'a>>,
}

impl <'a> Player<'a> {
    pub fn new(id: String, hand: Vec<HandCard<'a>>) -> Player<'a> {
        Player {
            id,
            hand
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_card_count(&self) -> usize {
        self.hand.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::*;

    #[test]
    fn it_has_an_id() {
        let id = "id1";;
        let player = Player::new(String::from("id1"), vec!());
        assert_eq!(player.get_id(), id);
    }

    #[test]
    fn it_shows_number_of_cards_left() {
        let id = String::from("id1");
        let suit_order = [
            Suit::Clubs,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Spades,
        ];
        let suits = get_suit_array(&suit_order);
        let deck = Deck::new(1, 0, &suits);

        let dealt = deck.deal(4);
        let player = Player::new(id, dealt[0].to_owned()); 
        assert_eq!(player.get_card_count(), 13);
    }
}

