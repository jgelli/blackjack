use std::fmt;

use crate::card::{Card, CardValue};
use crate::card::{FACE_CARDS, NAIPES};
use crate::user_input::UserInput;

use rand::seq::SliceRandom;

const BLACKJACK: u8 = 21;

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Player,
    Table,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Turn::Player => write!(f, "Player"),
            Turn::Table => write!(f, "Table"),
        }
    }
}


pub struct Game {
    deck: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub table_hand: Vec<Card>,
    pub player_wallet: usize,
    pub player_bet: usize,
    turn: Turn,
    pub player_burst: bool,
}

impl Game {
    fn create_deck() -> Vec<Card> {
        let mut deck = Vec::new();

        for naipe in NAIPES {
            for card_number in 1..=10 {
                deck.push(Card {
                    naipe,
                    value: CardValue::Number(card_number),
                });
            }
            for face_card in FACE_CARDS {
                deck.push(Card {
                    naipe,
                    value: CardValue::Face(face_card),
                });
            }
        }
        deck
    }

    pub fn deck_len(&self) -> usize {
        self.deck.len()
    }

    fn create_game_deck(&mut self, num_decks: u8) {
        let mut rng = rand::rng();

        for _ in 0..num_decks {
            self.deck.extend(Game::create_deck());
        }
        self.deck.shuffle(&mut rng);
    }

    pub fn new(num_decks: u8, wallet_value: usize) -> Self {
        let mut game = Game {
            deck: Vec::new(),
            player_hand: Vec::new(),
            table_hand: Vec::new(),
            player_wallet: wallet_value,
            player_bet: 0,
            turn: Turn::Player,
            player_burst: false
        };
        game.create_game_deck(num_decks);
        return game;
    }

    pub fn reset(&mut self) {
        self.player_hand.clear();
        self.table_hand.clear();
        self.player_bet = 0;
        self.turn = Turn::Player;
        self.player_burst = false;
    }

    pub fn change_turn(&mut self) {
        match self.turn {
            Turn::Player => self.turn = Turn::Table,
            Turn::Table => self.turn = Turn::Player
        }
    }

    fn current_hand_mut(&mut self) -> &mut Vec<Card> {
        match self.turn {
            Turn::Player => &mut self.player_hand,
            Turn::Table => &mut self.table_hand,
        }
    }

    fn current_hand(&self) -> &Vec<Card> {
        match self.turn {
            Turn::Player => &self.player_hand,
            Turn::Table => &self.table_hand,
        }
    }

    // asking for another card
    pub fn hit_card_current_hand(&mut self) -> bool {
        if let Some(card) = self.deck.pop() {
            self.current_hand_mut().push(card);
            return true;
        }

        println!("Empty Deck!");
        return false;
    }

    pub fn process_round(&mut self) {
        let player_sum = self.calculate_hand(&self.player_hand);
        let table_sum = self.calculate_hand(&self.table_hand);

        if player_sum > BLACKJACK { //player lose
            self.player_wallet -= self.player_bet;
        } else if table_sum > BLACKJACK || player_sum > table_sum { // player win
            self.player_wallet += self.player_bet;
        } else if table_sum > player_sum {
            self.player_wallet -= self.player_bet;
        }

    }

    pub fn play_round(&mut self) {
        
        let mut hit_card = true;
        while hit_card {
            println!("------------ {} ------------", self.turn);
            self.hit_card_current_hand();
            self.print_hand(self.current_hand());

            let sum = self.calculate_hand(self.current_hand());
            println!("CARDS SUM: {sum}");

            if sum == BLACKJACK {
                break;
            } else if sum > BLACKJACK { //burst blackjack
                //check if the burst is the player
                self.player_burst = self.turn == Turn::Player;
                break;
            }

            if self.turn == Turn::Player {
                hit_card  = UserInput::condition("Do you want to hit one more card?");
            }

            if self.turn == Turn::Table {
                if sum >= 17 {
                    hit_card = false;
                }
            }
        }
        println!("\n\n\n");
    }

    pub fn calculate_hand(&self, hand: &Vec<Card>) -> u8 {
        let mut sum = 0;
        for card in hand {
            sum += card.numeric_value();
        }
        return sum;
    }

    pub fn is_burst(&self, hand: &Vec<Card>) -> bool {
        self.calculate_hand(hand) > BLACKJACK
    }

    pub fn print_hand(&self, hand: &Vec<Card>) {
        for card in hand {
            print!("|");
            print!("{}", card.display());
        }
        println!("|");
    }
}
