mod mouse_input_engine;
mod text_input_engine;

use std::io::Result;

pub use mouse_input_engine::MouseInputEngine;
pub use text_input_engine::TextInputEngine;

pub trait Engine {
    fn start(&mut self) -> Result<()>;
}
