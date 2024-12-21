use super::{renderer::Renderer, Game};
use std::io::{stdin, BufRead, BufReader};

const WELCOME_MESSAGE: &str = "Welcome to >>> Consolitaire <<<";
const MENU_MESSAGE: &str = "___Menu___
1. Click on deck
2. Move card from pile to stack I
3. Move card from pile to suit stack I
4. Move card from stack I to stack J
5. Move card from stack I to suit stack J
6. Move card from suit stack I to stack J
7. Restart
8. Exit
P.s. Type a number representing a command from menu to execute it
as well as its arguments, if they are present
For example: type '5 1 4' to move card from stack 1 to stack 4
";

enum Command {
    ClickOnDeck,
    MoveCardFromPileToStack(usize),
    MoveCardFromPileToSuitStack(usize),
    MoveCardFromStackToStack(usize, usize),
    MoveCardFromStackToSuitStack(usize, usize),
    MoveCardFromSuitStackToStack(usize, usize),
    Restart,
    Exit,
    Invalid,
}

pub struct TextInputEngine;
struct InputParser;

impl TextInputEngine {
    pub fn start<R: Renderer<Game>>(mut game: &mut Game) {
        println!("{WELCOME_MESSAGE}");
        println!("{MENU_MESSAGE}");
        Self::show_game::<R>(&game);

        let mut input = String::new();
        let mut reader = BufReader::new(stdin());
        loop {
            input.clear();
            match reader.read_line(&mut input) {
                Ok(_) => {
                    let command = InputParser::parse(&input);

                    match command {
                        Command::ClickOnDeck => Self::click_on_deck(&mut game),
                        Command::MoveCardFromPileToStack(i) => {
                            Self::move_card_from_pile_to_stack(&mut game, i)
                        }
                        Command::MoveCardFromPileToSuitStack(i) => {
                            Self::move_card_from_pile_to_suit_stack(&mut game, i)
                        }
                        Command::MoveCardFromStackToStack(i, j) => {
                            Self::move_card_from_stack_to_stack(&mut game, i, j)
                        }
                        Command::MoveCardFromStackToSuitStack(i, j) => {
                            Self::move_card_from_stack_to_suit_stack(&mut game, i, j)
                        }
                        Command::MoveCardFromSuitStackToStack(i, j) => {
                            Self::move_card_from_suit_stack_to_stack(&mut game, i, j)
                        }
                        Command::Exit => {
                            println!(">> Exiting the game");
                            break;
                        }
                        Command::Restart => {
                            Self::restart(&mut game);
                        }
                        Command::Invalid => {
                            println!(">> Invalid command")
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {error}");
                    break;
                }
            }

            Self::show_game::<R>(&game);
        }

        println!(">> The game was exited");
    }

    fn show_game<R: Renderer<Game>>(game: &Game) {
        println!("----- Table -----");
        if let Err(error) = R::render(game) {
            eprintln!("Error rendering game: {error}");
        }
    }

    fn click_on_deck(game: &mut Game) {
        println!(">> You clicked on the deck");
        game.move_cards_from_deck_to_pile();
    }

    fn move_card_from_pile_to_stack(game: &mut Game, i: usize) {
        println!(">> Trying to move the card from pile to stack {i}");
        let mut status = "Fail";
        if i > 0 {
            if game.move_card_from_pile_to_stack(i - 1) {
                status = "Success";
            }
        }
        println!(">> {status}");
    }

    fn move_card_from_pile_to_suit_stack(game: &mut Game, i: usize) {
        println!(">> Trying to move the card from pile to suit stack {i}");
        let mut status = "Fail";
        if i > 0 {
            if game.move_card_from_pile_to_suit_stack(i - 1) {
                status = "Success";
            }
        }
        println!(">> {status}");
    }

    fn move_card_from_stack_to_stack(game: &mut Game, i: usize, j: usize) {
        println!(">> Trying to move the card from stack {i} to stack {j}");
        let mut status = "Fail";
        if i > 0 && j > 0 {
            if game.move_card_from_stack_to_stack(i - 1, j - 1) {
                status = "Success";
            }
        }
        println!(">> {status}");
    }

    fn move_card_from_stack_to_suit_stack(game: &mut Game, i: usize, j: usize) {
        println!(">> Trying to move the card from stack {i} to suit stack {j}");
        let mut status = "Fail";
        if i > 0 && j > 0 {
            if game.move_card_from_stack_to_suit_stack(i - 1, j - 1) {
                status = "Success";
            }
        }
        println!(">> {status}");
    }

    fn move_card_from_suit_stack_to_stack(game: &mut Game, i: usize, j: usize) {
        println!(">> Trying to move the card from suit stack {i} to stack {j}");
        let mut status = "Fail";
        if i > 0 && j > 0 {
            if game.move_card_from_suit_stack_to_stack(i - 1, j - 1) {
                status = "Success";
            }
        }
        println!(">> {status}");
    }

    fn restart(game: &mut Game) {
        println!(">> Restarting the game");
        game.restart();
    }
}

impl InputParser {
    fn parse(input: &str) -> Command {
        let tokens = input.split_ascii_whitespace().map(|x| x.parse::<usize>());

        let is_valid = tokens.clone().all(|x| x.is_ok());

        if is_valid {
            let tokens = tokens.map(Result::unwrap).collect::<Vec<_>>();
            return match &tokens[..] {
                [1] => Command::ClickOnDeck,
                [7] => Command::Restart,
                [8] => Command::Exit,
                [2, i] => Command::MoveCardFromPileToStack(*i),
                [3, i] => Command::MoveCardFromPileToSuitStack(*i),
                [4, i, j] => Command::MoveCardFromStackToStack(*i, *j),
                [5, i, j] => Command::MoveCardFromStackToSuitStack(*i, *j),
                [6, i, j] => Command::MoveCardFromSuitStackToStack(*i, *j),
                _ => Command::Invalid,
            };
        }

        Command::Invalid
    }
}
