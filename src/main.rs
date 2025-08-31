mod board;
mod dice;
mod game;
mod player;
mod ui;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}