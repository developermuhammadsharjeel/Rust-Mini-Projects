use crate::board::Board;
use crate::player::{Player, PlayerId};
use colored::{Color, Colorize};
use std::collections::HashMap;
use std::io::{self, Write};

pub struct UI {
    input_buffer: String,
}

impl UI {
    pub fn new() -> Self {
        UI {
            input_buffer: String::new(),
        }
    }
    
    fn get_input(&mut self) -> String {
        self.input_buffer.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut self.input_buffer).unwrap();
        let input = self.input_buffer.trim().to_string();
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            println!("Quitting game. Goodbye!");
            std::process::exit(0);
        }
        input
    }
    
    pub fn display_welcome(&self) {
        println!("{}", "Welcome to Ludo Game!".bold().green());
        println!("Get all your pieces from the yard to the finish line.");
        println!("Roll a 6 to move a piece out of the yard.");
        println!("Capture opponent pieces by landing on their space.");
        println!("Roll a 6 or capture to get an extra turn.\n");
    }
    
    pub fn get_player_count(&mut self) -> usize {
        loop {
            print!("Enter the number of players (2-4): ");
            let input = self.get_input();
            
            match input.parse::<usize>() {
                Ok(count) if count >= 2 && count <= 4 => return count,
                _ => println!("Please enter a number between 2 and 4."),
            }
        }
    }
    
    pub fn get_player_name(&mut self, player_id: PlayerId) -> String {
        print!("Enter name for Player {}: ", player_id + 1);
        let name = self.get_input();
        
        if name.is_empty() {
            format!("Player {}", player_id + 1)
        } else {
            name
        }
    }
    
    pub fn display_board(&self, board: &Board, player_colors: &HashMap<PlayerId, Color>) {
        println!("{}", board.render(player_colors));
    }
    
    pub fn display_player_turn(&self, player: &Player) {
        println!("\n{}'s turn", player.name.color(player.color).bold());
    }
    
    pub fn prompt_for_dice_roll(&self) {
        print!("Press Enter to roll the dice...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    
    pub fn display_dice_roll(&self, value: u8) {
        println!("You rolled a {}!", value.to_string().yellow().bold());
    }
    
    pub fn choose_piece(&mut self, player_id: PlayerId, valid_pieces: &[usize]) -> usize {
        println!("Choose a piece to move:");
        
        for (i, &piece_idx) in valid_pieces.iter().enumerate() {
            println!("{}. Piece {}", i + 1, piece_idx);
        }
        
        loop {
            print!("Enter choice (1-{}): ", valid_pieces.len());
            let input = self.get_input();
            
            match input.parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= valid_pieces.len() => {
                    return valid_pieces[choice - 1];
                }
                _ => println!("Invalid choice. Please try again."),
            }
        }
    }
    
    pub fn display_move_result(&self, player_id: PlayerId, piece_idx: usize, message: &str) {
        println!("Player {}'s piece {} {}", player_id, piece_idx, message);
    }
    
    pub fn display_no_valid_moves(&self) {
        println!("{}", "No valid moves available. Turn skipped.".yellow());
    }
    
    pub fn display_extra_turn(&self) {
        println!("{}", "You get an extra turn!".green());
    }
    
    pub fn display_game_over(&self, winner: &Player) {
        println!("\n{}", "=== GAME OVER ===".bold());
        println!("{} {} {}", 
            "Congratulations!".green().bold(), 
            winner.name.color(winner.color).bold(), 
            "has won the game!".green().bold()
        );
    }
}