use std::io::Result;

use super::{renderer::GameRenderer, Game, GameObject};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
};

pub struct GameEngine<'a> {
    renderer: GameRenderer,
    game: &'a mut Game,
}

enum GameEvent {
    MouseDrag { row: u16, column: u16 },
    MouseDown { row: u16, column: u16 },
    MouseUp { row: u16, column: u16 },
    Restart,
    Exit,
    None,
}

fn wait_for_event() -> GameEvent {
    match event::read() {
        Ok(event) => match event {
            Event::Key(key_event) => match key_event {
                KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                } => GameEvent::Exit,
                KeyEvent {
                    code: KeyCode::Char('r'),
                    kind: KeyEventKind::Press,
                    ..
                } => GameEvent::Restart,
                _ => GameEvent::None,
            },
            Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent {
                    row, column, kind, ..
                } => match kind {
                    MouseEventKind::Drag(MouseButton::Left) => GameEvent::MouseDrag { row, column },
                    MouseEventKind::Down(MouseButton::Left) => GameEvent::MouseDown { row, column },
                    MouseEventKind::Up(MouseButton::Left) => GameEvent::MouseUp { row, column },
                    _ => GameEvent::None,
                },
            },
            _ => GameEvent::None,
        },
        Err(error) => {
            eprintln!("Something goes wrong: {error}");
            GameEvent::Exit
        }
    }
}

impl<'a> GameEngine<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        let renderer = GameRenderer::new();
        Self { renderer, game }
    }

    pub fn start(&mut self) -> Result<()> {
        self.renderer.init()?;

        self.render_game();

        loop {
            let event = wait_for_event();

            let should_rerender = match event {
                GameEvent::MouseDown { row, column } => self.on_mouse_down(row, column),
                GameEvent::MouseUp { row, column } => self.on_mouse_up(row, column),
                GameEvent::MouseDrag { row, column } => self.on_mouse_drag(row, column),
                GameEvent::Exit => break,
                GameEvent::Restart => {
                    self.restart();
                    true
                }
                GameEvent::None => false,
            };

            if should_rerender {
                self.render_game();
            }
        }

        self.renderer.cleanup()?;

        Ok(())
    }

    fn render_game(&mut self) {
        if let Err(error) = self.renderer.render(&self.game) {
            eprintln!("Error rendering game: {error}");
        }
    }

    fn restart(&mut self) {
        self.game.restart();
    }

    fn on_mouse_down(&mut self, row: u16, column: u16) -> bool {
        let object = self.renderer.get_object_at(&self.game, row, column);
        match object {
            GameObject::Deck => {
                self.on_click_on_deck();
            }
            GameObject::Pile | GameObject::LastCardOfStack(_) | GameObject::SuitStack(_) => {
                self.renderer.select_object(&self.game, object);
                self.renderer.set_selected_object_position(row, column);
            }
            _ => {}
        }

        match object {
            GameObject::Deck => true,
            _ => false,
        }
    }

    fn on_mouse_up(&mut self, row: u16, column: u16) -> bool {
        let selected_object = self.renderer.get_selected_object();
        if selected_object.is_none() {
            return false;
        }

        let target_object = self.renderer.get_object_at(&self.game, row, column);

        match (selected_object, target_object) {
            (GameObject::Pile, GameObject::SuitStack(i)) => {
                self.game.move_card_from_pile_to_suit_stack(i as usize);
            }
            (GameObject::Pile, GameObject::LastCardOfStack(i)) => {
                self.game.move_card_from_pile_to_stack(i as usize);
            }
            (GameObject::SuitStack(i), GameObject::LastCardOfStack(j)) => {
                self.game
                    .move_card_from_suit_stack_to_stack(i as usize, j as usize);
            }
            (GameObject::LastCardOfStack(i), GameObject::SuitStack(j)) => {
                self.game
                    .move_card_from_stack_to_suit_stack(i as usize, j as usize);
            }
            (GameObject::LastCardOfStack(i), GameObject::LastCardOfStack(j)) => {
                self.game
                    .move_card_from_stack_to_stack(i as usize, j as usize);
            }
            _ => {}
        }

        self.renderer.select_object(&self.game, GameObject::None);

        true
    }

    fn on_mouse_drag(&mut self, row: u16, column: u16) -> bool {
        self.renderer.set_selected_object_position(row, column);

        !self.renderer.get_selected_object().is_none()
    }

    fn on_click_on_deck(&mut self) {
        self.game.move_cards_from_deck_to_pile();
    }
}
