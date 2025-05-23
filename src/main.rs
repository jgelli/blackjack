use blackjack::game::Game;
use blackjack::user_input::UserInput;



fn main() {
    let mut game = Game::new(4, 100);

    loop {
        if game.deck_len() < 20 {
            println!("The deck has less than 20 cards! Game Over");
            break
        }

        if game.player_wallet == 0 {
            println!("You are broken! Wallet: U$ 0");
            break;
        }

        println!("Wallet: U$ {}\n\n", game.player_wallet);

        if !UserInput::condition("Do you want to bet?") {
            println!("Bye bye!");
            break;
        } 

        game.player_bet = UserInput::get_bet(game.player_wallet);
        
        //Player round
        game.play_round();

        if !game.player_burst {
            //change to table's turn
            game.change_turn();
            game.play_round();            
        }


        game.process_round();

        game.reset();

    }
}




