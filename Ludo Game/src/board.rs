use crate::player::{Piece, PlayerId};
use colored::{ColoredString, Colorize};
use std::collections::HashMap;

// Constants for the board
pub const HOME_SPACES: usize = 6;
pub const MAIN_TRACK_SPACES: usize = 52;
pub const TOTAL_PIECES_PER_PLAYER: usize = 4;

// Board positions type aliases
pub type Position = usize;
pub type HomePosition = usize;

#[derive(Debug)]
pub struct Board {
    // Main track - shared by all players
    main_track: Vec<Vec<(PlayerId, usize)>>, // Player ID and piece index
    
    // Home tracks - one per player
    home_tracks: HashMap<PlayerId, Vec<Option<usize>>>, // Maps player ID to their home track pieces
    
    // Yard positions - where pieces start and return when captured
    yards: HashMap<PlayerId, Vec<bool>>, // true if piece is in yard
    
    // Finished positions - destination for pieces
    finished: HashMap<PlayerId, Vec<bool>>, // true if piece is finished
    
    // Starting positions for each player on the main track
    player_starts: HashMap<PlayerId, Position>,
}

impl Board {
    pub fn new(player_count: usize) -> Self {
        let mut main_track = vec![Vec::new(); MAIN_TRACK_SPACES];
        let mut home_tracks = HashMap::new();
        let mut yards = HashMap::new();
        let mut finished = HashMap::new();
        let mut player_starts = HashMap::new();
        
        // Set up player-specific data
        for player_id in 0..player_count {
            home_tracks.insert(player_id, vec![None; HOME_SPACES]);
            yards.insert(player_id, vec![true; TOTAL_PIECES_PER_PLAYER]); // All pieces start in yard
            finished.insert(player_id, vec![false; TOTAL_PIECES_PER_PLAYER]);
            
            // Starting positions are evenly distributed around the board
            let start_pos = (player_id * (MAIN_TRACK_SPACES / player_count)) % MAIN_TRACK_SPACES;
            player_starts.insert(player_id, start_pos);
        }
        
        Board {
            main_track,
            home_tracks,
            yards,
            finished,
            player_starts,
        }
    }
    
    // Check if a piece is in the yard
    pub fn is_in_yard(&self, player_id: PlayerId, piece_idx: usize) -> bool {
        self.yards[&player_id][piece_idx]
    }
    
    // Check if a piece has finished
    pub fn is_finished(&self, player_id: PlayerId, piece_idx: usize) -> bool {
        self.finished[&player_id][piece_idx]
    }
    
    // Get piece's location type
    pub fn get_piece_location(&self, player_id: PlayerId, piece_idx: usize) -> PieceLocation {
        if self.is_in_yard(player_id, piece_idx) {
            return PieceLocation::Yard;
        }
        
        if self.is_finished(player_id, piece_idx) {
            return PieceLocation::Finished;
        }
        
        // Check main track
        for (pos, pieces) in self.main_track.iter().enumerate() {
            for &(pid, idx) in pieces {
                if pid == player_id && idx == piece_idx {
                    return PieceLocation::MainTrack(pos);
                }
            }
        }
        
        // Check home track
        let home_track = &self.home_tracks[&player_id];
        for (pos, &piece_opt) in home_track.iter().enumerate() {
            if let Some(idx) = piece_opt {
                if idx == piece_idx {
                    return PieceLocation::HomeTrack(pos);
                }
            }
        }
        
        // This should never happen if the board state is valid
        panic!("Piece not found on board: Player {}, Piece {}", player_id, piece_idx);
    }
    
    // Move a piece from yard to start position
    pub fn move_from_yard_to_start(&mut self, player_id: PlayerId, piece_idx: usize) -> bool {
        if !self.is_in_yard(player_id, piece_idx) {
            return false;
        }
        
        let start_pos = self.player_starts[&player_id];
        
        // Update yard status
        self.yards.get_mut(&player_id).unwrap()[piece_idx] = false;
        
        // Add to main track
        self.main_track[start_pos].push((player_id, piece_idx));
        
        true
    }
    
    // Move a piece on the main track
    pub fn move_piece(&mut self, player_id: PlayerId, piece_idx: usize, steps: usize) -> MoveResult {
        match self.get_piece_location(player_id, piece_idx) {
            PieceLocation::Yard => {
                // Can only move out of yard with a 6
                if steps == 6 {
                    self.move_from_yard_to_start(player_id, piece_idx);
                    MoveResult::Moved
                } else {
                    MoveResult::InvalidMove
                }
            },
            PieceLocation::MainTrack(curr_pos) => {
                // Remove from current position
                let position = &mut self.main_track[curr_pos];
                let index = position.iter().position(|&(pid, idx)| pid == player_id && idx == piece_idx).unwrap();
                position.remove(index);
                
                // Calculate new position
                let start_pos = self.player_starts[&player_id];
                let absolute_pos = (curr_pos + steps) % MAIN_TRACK_SPACES;
                
                // Check if we've gone around the board and need to enter home track
                let distance_from_start = (MAIN_TRACK_SPACES + absolute_pos - start_pos) % MAIN_TRACK_SPACES;
                let home_entry_distance = MAIN_TRACK_SPACES - HOME_SPACES;
                
                if distance_from_start > home_entry_distance {
                    // Enter home track
                    let home_pos = distance_from_start - home_entry_distance - 1;
                    
                    if home_pos < HOME_SPACES {
                        // Valid home track position
                        if self.home_tracks[&player_id][home_pos].is_some() {
                            // Space already occupied by own piece
                            return MoveResult::InvalidMove;
                        }
                        
                        self.home_tracks.get_mut(&player_id).unwrap()[home_pos] = Some(piece_idx);
                        return MoveResult::Moved;
                    } else {
                        // Overshooting home track
                        return MoveResult::InvalidMove;
                    }
                }
                
                // Check for captures
                let mut captured = false;
                if !self.main_track[absolute_pos].is_empty() {
                    let mut i = 0;
                    while i < self.main_track[absolute_pos].len() {
                        let (other_pid, other_idx) = self.main_track[absolute_pos][i];
                        
                        if other_pid != player_id {
                            // Capture other player's piece
                            self.main_track[absolute_pos].remove(i);
                            self.yards.get_mut(&other_pid).unwrap()[other_idx] = true;
                            captured = true;
                        } else {
                            i += 1;
                        }
                    }
                }
                
                // Add to new position
                self.main_track[absolute_pos].push((player_id, piece_idx));
                
                if captured {
                    MoveResult::Captured
                } else {
                    MoveResult::Moved
                }
            },
            PieceLocation::HomeTrack(home_pos) => {
                let new_pos = home_pos + steps;
                
                if new_pos >= HOME_SPACES {
                    // Finish the piece if exact
                    if new_pos == HOME_SPACES {
                        self.home_tracks.get_mut(&player_id).unwrap()[home_pos] = None;
                        self.finished.get_mut(&player_id).unwrap()[piece_idx] = true;
                        MoveResult::Finished
                    } else {
                        // Can't overshoot finish
                        MoveResult::InvalidMove
                    }
                } else {
                    // Move within home track
                    if self.home_tracks[&player_id][new_pos].is_some() {
                        // Space already occupied
                        return MoveResult::InvalidMove;
                    }
                    
                    self.home_tracks.get_mut(&player_id).unwrap()[home_pos] = None;
                    self.home_tracks.get_mut(&player_id).unwrap()[new_pos] = Some(piece_idx);
                    MoveResult::Moved
                }
            },
            PieceLocation::Finished => MoveResult::InvalidMove,
        }
    }
    
    // Check if a player has won
    pub fn has_won(&self, player_id: PlayerId) -> bool {
        self.finished[&player_id].iter().all(|&finished| finished)
    }
    
    // Render the board as a string
    pub fn render(&self, player_colors: &HashMap<PlayerId, colored::Color>) -> String {
        let mut output = String::new();
        
        // Display board header
        output.push_str("\n=== LUDO BOARD ===\n\n");
        
        // Render main track
        output.push_str("Main Track:\n");
        for i in 0..MAIN_TRACK_SPACES {
            let pos_str = if self.main_track[i].is_empty() {
                format!("{:2}", i).normal()
            } else {
                let (player_id, piece_idx) = self.main_track[i][0];
                let color = player_colors[&player_id];
                format!("P{}{}", player_id, piece_idx).color(color)
            };
            
            output.push_str(&format!("[{}]", pos_str));
            
            if (i + 1) % 13 == 0 {
                output.push('\n');
            }
        }
        
        // Render player information
        output.push_str("\nPlayers:\n");
        
        for (&player_id, yard) in &self.yards {
            let color = player_colors[&player_id];
            output.push_str(&format!("Player {}: ", player_id).color(color).to_string());
            
            // Yard pieces
            output.push_str("Yard: ");
            for (idx, &in_yard) in yard.iter().enumerate() {
                if in_yard {
                    output.push_str(&format!("{} ", idx).color(color).to_string());
                }
            }
            
            // Home track
            output.push_str("| Home: ");
            if let Some(home_track) = self.home_tracks.get(&player_id) {
                for (pos, &piece_opt) in home_track.iter().enumerate() {
                    if let Some(piece_idx) = piece_opt {
                        output.push_str(&format!("{}:{} ", pos, piece_idx).color(color).to_string());
                    }
                }
            }
            
            // Finished pieces
            output.push_str("| Finished: ");
            if let Some(finished_pieces) = self.finished.get(&player_id) {
                for (idx, &is_finished) in finished_pieces.iter().enumerate() {
                    if is_finished {
                        output.push_str(&format!("{} ", idx).color(color).to_string());
                    }
                }
            }
            
            output.push('\n');
        }
        
        output
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceLocation {
    Yard,
    MainTrack(Position),
    HomeTrack(HomePosition),
    Finished,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    Moved,
    Captured,
    Finished,
    InvalidMove,
}