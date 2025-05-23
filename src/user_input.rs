use std::io::stdin;


pub struct UserInput;

impl UserInput {
    fn get_input() -> String {
        let mut player_input = String::new();
    
        stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line");
    
        player_input.trim().to_string()
    }

    pub fn condition(msg: &str) -> bool {
        println!("{msg}? (y/n)");
        
        let input = Self::get_input();
        input.eq_ignore_ascii_case("y")
    }

    pub fn get_bet(max: usize) -> usize {
        loop {
            println!("How much do you want bet? (min. 1 - max. {max})");
            let input = Self::get_input();
            if let Ok(parsed_bet) = input.trim().parse::<usize>() {
                if parsed_bet <= 0 || parsed_bet > max {
                    println!("Invalid bet number!");
                    continue;
                }

                return parsed_bet;
            }
            
            println!("\n\nInvalid input. Please enter a positive number between 1 or {max}.\n\n");
        }
    }
}