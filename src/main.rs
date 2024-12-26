mod game;

use game::Game;

fn main() {
    let mut game = Game::default();
    game.start();
}
