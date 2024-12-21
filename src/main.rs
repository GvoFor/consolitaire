mod game;
use std::env::{self, Args};

use game::{
    engine::TextInputEngine,
    renderer::{ASCIIRenderer, TextOutputRenderer},
    Game,
};

enum GameVersion {
    V1,
    V2,
    Default,
}

fn main() {
    let mut game = Game::default();

    let version = parse_env_args(env::args());
    match version {
        GameVersion::V1 => start_v1(&mut game),
        GameVersion::V2 => start_v2(&mut game),
        GameVersion::Default => start_v2(&mut game),
    }
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

fn start_v1(game: &mut Game) {
    println!("Starting game verion 1");
    TextInputEngine::start::<TextOutputRenderer>(game);
}

fn start_v2(game: &mut Game) {
    println!("Starting game verion 2");
    TextInputEngine::start::<ASCIIRenderer>(game);
}
