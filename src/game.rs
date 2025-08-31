use crate::board::{Board, MoveResult, TOTAL_PIECES_PER_PLAYER};
use crate::dice::Dice;
use crate::player::{Piece, Player, PlayerId};
use crate::ui::UI;
use colored::Color;
use std::collections::HashMap;

pub struct Game {
    players: Vec<Player>,
    board: Board,
    dice: Dice,
    current_player_idx: usize,
    ui: UI,
    player_colors: HashMap<PlayerId, Color>,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
    let mut ui = UI::new();
    let player_count = ui.get_player_count();
        
        let colors = vec![
            Color::Red, 
            Color::Green, 
            Color::Blue, 
            Color::Yellow
        ];
        
        let mut players = Vec::new();
        let mut player_colors = HashMap::new();
        
        for i in 0..player_count {
            let name = ui.get_player_name(i);
            let color = colors[i % colors.len()];
            player_colors.insert(i, color);
            players.push(Player::new(i, name, color));
        }
        
        Game {
            players,
            board: Board::new(player_count),
            dice: Dice::default(),
            current_player_idx: 0,
            ui,
            player_colors,
            game_over: false,
        }
    }
    
    pub fn start(&mut self) {
        self.ui.display_welcome();
        
        while !self.game_over {
            self.play_turn();
        }
        
        self.ui.display_game_over(&self.players[self.current_player_idx]);
    }
    
    fn play_turn(&mut self) {
        let current_player = &self.players[self.current_player_idx];
        
        self.ui.display_board(&self.board, &self.player_colors);
        self.ui.display_player_turn(current_player);
        
        // Roll dice
        self.ui.prompt_for_dice_roll();
        let dice_value = self.dice.roll();
        self.ui.display_dice_roll(dice_value);
        
        // Get valid pieces that can move
        let mut valid_pieces = Vec::new();
        
        for piece_idx in 0..TOTAL_PIECES_PER_PLAYER {
            let location = self.board.get_piece_location(current_player.id, piece_idx);
            
            match location {
                crate::board::PieceLocation::Yard if dice_value == 6 => {
                    valid_pieces.push(piece_idx);
                }
                crate::board::PieceLocation::MainTrack(_) | crate::board::PieceLocation::HomeTrack(_) => {
                    valid_pieces.push(piece_idx);
                }
                _ => {}
            }
        }
        
        // If there are no valid pieces to move, skip turn
        if valid_pieces.is_empty() {
            self.ui.display_no_valid_moves();
            self.next_player();
            return;
        }
        
        // Let player choose a piece to move
        let chosen_piece = self.ui.choose_piece(current_player.id, &valid_pieces);
        
        // Move the piece
        let result = self.board.move_piece(current_player.id, chosen_piece, dice_value as usize);
        
        match result {
            MoveResult::Moved => {
                self.ui.display_move_result(current_player.id, chosen_piece, "moved successfully");
            }
            MoveResult::Captured => {
                self.ui.display_move_result(current_player.id, chosen_piece, "captured an opponent's piece");
                // Player gets another turn after capturing
                return;
            }
            MoveResult::Finished => {
                self.ui.display_move_result(current_player.id, chosen_piece, "reached the finish");
                
                // Check if player has won
                if self.board.has_won(current_player.id) {
                    self.game_over = true;
                    return;
                }
            }
            MoveResult::InvalidMove => {
                self.ui.display_move_result(current_player.id, chosen_piece, "couldn't move (invalid move)");
            }
        }
        
        // If player rolled a 6, they get another turn
        if dice_value == 6 && !self.game_over {
            self.ui.display_extra_turn();
            return;
        }
        
        self.next_player();
    }
    
    fn next_player(&mut self) {
        self.current_player_idx = (self.current_player_idx + 1) % self.players.len();
    }
}