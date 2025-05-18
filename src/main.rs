mod card;
mod deck;

use card::Card;
use deck::create_blackjack_deck;

use std::io::stdin;

const BLACKJACK: u8 = 21;


fn main() {
    let mut table_deck = create_blackjack_deck();
    let mut table_hand: Vec<Card> = Vec::new();
    let mut player_hand: Vec<Card> = Vec::new();
    let mut player_wallet: usize = 100;

    'game_loop: loop {
        let mut player_sum: u8 = 0;
        let mut table_sum: u8 = 0;

        if player_wallet == 0 {
            println!("You are broken! Wallet: U$ {player_wallet}");
            break;
        }

        println!("Wallet: U$ {player_wallet}");
        println!("How much do you want bet? Or type 'exit' to leave!");

        let player_input = scanner();
        if player_input.eq_ignore_ascii_case("exit") {
            println!("Thanks for playing! Final wallet: {player_wallet}");
            break;
        }

        let bet: usize;
        if let Ok(parsed_bet) = player_input.parse::<usize>() {
            if parsed_bet <= 0 || parsed_bet > player_wallet {
                println!("Invalid bet number! (min. 1)(max. {player_wallet})");
                continue;
            }

            bet = parsed_bet;
        } else {
            println!("\n\nInvalid input. Please enter a positive number between 1 or {player_wallet} or 'exit'.\n\n");
            continue;
        }

        println!("valid bet");
        player_wallet -= bet;

        'player_loop: loop {
            if let Some(card) = table_deck.pop() {
                player_sum += card.numeric_value();
                println!("Player - {card:?}");
                println!("Player - SUM: {player_sum}\n\n")
            } else {
                println!("End of the deck. End of the game!");
                break 'game_loop;
            }

            if player_sum == BLACKJACK {
                break 'player_loop;
            } else if player_sum > BLACKJACK {
                println!("Player - LOSE U$ {bet}!");
                continue 'game_loop;
            }

            println!("\n\nOne more card? Y/N");
            let player_input = scanner();
            if player_input.eq_ignore_ascii_case("n") {
                break 'player_loop;
            }
        }

        'table_loop: loop {
            if let Some(card) = table_deck.pop() {
                table_sum += card.numeric_value();
                println!("Table - {card:?}");
                println!("Table - SUM: {table_sum}")
            } else {
                println!("End of the deck. End of the game!");
                break 'game_loop;
            }

            if table_sum > BLACKJACK {
                println!("Table - LOSE\n\n");

                let pot = bet * 2;
                player_wallet += pot;
                println!("Player WIN - U$ {pot}"); 
                continue 'game_loop;
            } else if table_sum > player_sum {
                println!("Table - WIN\n\n");
                continue 'game_loop;
            }else if table_sum == BLACKJACK {
                break 'table_loop;
            } 
        }

    }
}

fn scanner() -> String {
    let mut player_input = String::new();

    stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line");

    player_input.trim().to_string()
}
