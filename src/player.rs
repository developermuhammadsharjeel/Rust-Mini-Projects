use colored::Color;

pub type PlayerId = usize;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub color: Color,
}

impl Player {
    pub fn new(id: PlayerId, name: String, color: Color) -> Self {
        Player { id, name, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub player_id: PlayerId,
    pub index: usize,
}

impl Piece {
    pub fn new(player_id: PlayerId, index: usize) -> Self {
        Piece { player_id, index }
    }
}