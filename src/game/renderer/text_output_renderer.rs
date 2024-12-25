use std::io::Result;

use super::Renderer;
use crate::game::{shared::GameObject, Game};

pub struct TextOutputRenderer;

impl Renderer for TextOutputRenderer {
    fn render(&mut self, game: &Game) -> Result<()> {
        let deck = &game.deck;
        println!("Deck: {deck}");

        let pile = &game.pile;
        println!("Pile: {pile}");

        println!("");

        let suit_stacks = &game.suit_stacks;
        for (i, stack) in suit_stacks.iter().enumerate() {
            println!("Suit stack {}: {stack}", i + 1);
        }

        println!("");

        let stacks = &game.stacks;
        for (i, stack) in stacks.iter().enumerate() {
            println!("Stack {}: {stack}", i + 1);
        }

        Ok(())
    }

    fn get_object_at(&self, _game: &Game, _row: u16, _column: u16) -> GameObject {
        GameObject::None
    }

    fn get_selected_object(&self) -> GameObject {
        GameObject::None
    }

    fn select_object(&mut self, _game: &Game, _object: GameObject) {}

    fn set_selected_object_position(&mut self, _row: u16, _column: u16) {}
}
