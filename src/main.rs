mod game;
use game::{engine::TextInputEngine, Game};

fn main() {
    let game = Game::default();
    let mut engine = TextInputEngine::new(game);
    engine.start();
}
