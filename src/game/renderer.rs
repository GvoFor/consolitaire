mod ascii_renderer;
mod text_output_renderer;

use std::io::Result;

pub use ascii_renderer::ASCIIRenderer;
pub use text_output_renderer::TextOutputRenderer;

use super::{shared::GameObject, Game};

pub trait Renderer {
    fn render(&mut self, game: &Game) -> Result<()>;
    fn get_object_at(&self, game: &Game, row: u16, column: u16) -> GameObject;
    fn get_selected_object(&self) -> GameObject;
    fn select_object(&mut self, game: &Game, object: GameObject);
    fn set_selected_object_position(&mut self, row: u16, column: u16);
}
