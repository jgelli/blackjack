use crate::card::{Card, CardValue};
use crate::card::{NAIPES, FACE_CARDS};

use rand::seq::SliceRandom;
use rand::Rng;



fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for naipe in NAIPES {
        for card_number in 1..=10 {
            deck.push(Card {
                naipe,
                value: CardValue::Number(card_number)
            });
        }
        for face_card in FACE_CARDS {
            deck.push(Card {
                naipe,
                value: CardValue::Face(face_card)
            });
        }
    }
    deck
}

pub fn create_blackjack_deck() -> Vec<Card> {
    let mut rng = rand::rng();

    let mut deck = Vec::new();
    for _ in 1..=4 {
        deck.extend(create_deck());
    }
    deck.shuffle(&mut rng);
    return deck;
}