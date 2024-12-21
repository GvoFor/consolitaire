mod ascii_renderer;
mod text_output_renderer;

use std::io::Result;

pub use ascii_renderer::ASCIIRenderer;
pub use text_output_renderer::TextOutputRenderer;

pub trait Renderer<T> {
    fn render(t: &T) -> Result<()>;
}
