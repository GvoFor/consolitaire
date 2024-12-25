mod game;
use std::{
    env::{self, Args},
    io::{stdout, Stdout},
};

use game::{
    engine::{Engine, MouseInputEngine, TextInputEngine},
    renderer::{ASCIIRenderer, TextOutputRenderer},
    Game,
};

enum GameVersion {
    V1,
    V2,
    Default,
}

fn main() {
    let game = Game::default();
    let out = stdout();

    let version = parse_env_args(env::args());
    match version {
        GameVersion::V1 => start_game_v1(game),
        GameVersion::V2 => start_game_v2(game, out),
        GameVersion::Default => start_game_v2(game, out),
    };
}

fn parse_env_args(args: Args) -> GameVersion {
    if let Some(v) = args.skip(1).take(1).last() {
        match &v[..] {
            "v1" => GameVersion::V1,
            "v2" => GameVersion::V2,
            _ => GameVersion::Default,
        }
    } else {
        GameVersion::Default
    }
}

fn start_game_v1(mut game: Game) {
    println!("Starting game verion 1");
    let renderer = TextOutputRenderer;
    let mut engine = TextInputEngine::new(&mut game, renderer);
    if let Err(error) = engine.start() {
        println!("Error during the game: {error}");
    }
}

fn start_game_v2(mut game: Game, mut out: Stdout) {
    println!("Starting game verion 2");
    let renderer = ASCIIRenderer::new(&mut out);
    let mut engine = MouseInputEngine::new(&mut game, renderer);
    if let Err(error) = engine.start() {
        println!("Error during the game: {error}");
    }
}
