use super::{
    card::Card,
    card_collections::{Deck, Pile, Stack},
    Game, GameObject,
};

use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{size, Clear, ClearType, SetSize},
};
use std::{
    io::{stdout, Result, Stdout},
    ops::Div,
};

const PADDING_X: u16 = 2;
const PADDING_Y: u16 = 1;
const TABLE_COLS: u16 = 78;
const TABLE_ROWS: u16 = 35;
const CARD_COLS: u16 = 7;
const CARD_ROWS: u16 = 5;
const GAP_BETWEEN_STACK_CARDS: u16 = 3;
const GAP_BETWEEN_PILE_AND_DECK: u16 = 6;
const EMPTY_DECK_X: u16 = TABLE_COLS - CARD_COLS;
const EMPTY_DECK_Y: u16 = 0;
const PILE_LAST_CARD_X: u16 = EMPTY_DECK_X - 1 - GAP_BETWEEN_PILE_AND_DECK - CARD_COLS;
const PILE_LAST_CARD_Y: u16 = 0;
const FIRST_SUIT_STACK_X: u16 = 0;
const FIRST_SUIT_STACK_Y: u16 = 0;
const FIRST_STACK_X: u16 = FIRST_SUIT_STACK_X;
const FIRST_STACK_Y: u16 = FIRST_SUIT_STACK_Y + CARD_ROWS + 1;
const STACK_CARD_X_OFFSET: u16 = 1;
const STACK_CARD_Y_OFFSET: u16 = 2;

pub struct GameRenderer {
    out: Stdout,
    selected_object: GameObject,
    selected_object_row: u16,
    selected_object_column: u16,
    console_rows: Option<u16>,
    console_columns: Option<u16>,
}

impl GameRenderer {
    pub fn new() -> Self {
        Self {
            out: stdout(),
            selected_object: GameObject::None,
            selected_object_column: 0,
            selected_object_row: 0,
            console_rows: None,
            console_columns: None,
        }
    }

    pub fn init(&mut self) -> Result<()> {
        let (columns, rows) = size()?;
        self.console_columns = Some(columns);
        self.console_rows = Some(rows);

        let new_columns = TABLE_COLS + 2 * PADDING_X;
        let new_rows = TABLE_ROWS + 2 * PADDING_Y;
        execute!(self.out, EnableMouseCapture, SetSize(new_columns, new_rows))
    }

    pub fn cleanup(&mut self) -> Result<()> {
        match (self.console_rows, self.console_columns) {
            (Some(rows), Some(columns)) => {
                execute!(self.out, DisableMouseCapture, SetSize(columns, rows))
            }
            _ => execute!(self.out, DisableMouseCapture),
        }
    }

    pub fn render(&mut self, game: &Game) -> Result<()> {
        execute!(self.out, Clear(ClearType::All))?;

        for (i, stack) in game.suit_stacks.iter().enumerate() {
            self.render_suit_stack(stack, i as u16)?;
        }

        self.render_pile(&game.pile)?;

        self.render_deck(&game.deck)?;

        for (i, stack) in game.stacks.iter().enumerate() {
            self.render_stack(stack, i as u16)?;
        }

        self.render_selected_object(game)?;

        execute!(
            self.out,
            SetForegroundColor(Color::White),
            MoveTo(PADDING_X, PADDING_Y + TABLE_ROWS),
        )?;

        Ok(())
    }

    pub fn get_object_at(&self, game: &Game, row: u16, column: u16) -> GameObject {
        if is_point_outside_of_table(column, row) {
            return GameObject::None;
        }

        let (x, y) = (column - PADDING_X, row - PADDING_Y);

        let (deck_x, deck_y) = get_deck_position(&game.deck);
        if is_point_inside_card(x, y, deck_x, deck_y) {
            return GameObject::Deck;
        }

        if is_point_inside_card(x, y, PILE_LAST_CARD_X, PILE_LAST_CARD_Y) {
            return GameObject::Pile;
        }

        for i in 0..game.suit_stacks.len() {
            let (stack_x, stack_y) = get_suit_stack_position(i as u16);

            if is_point_inside_card(x, y, stack_x, stack_y) {
                return GameObject::SuitStack(i as u16);
            }
        }

        for i in 0..game.stacks.len() {
            let (last_card_x, last_card_y) = get_stack_last_card_position(&game.stacks, i as u16);

            if is_point_inside_card(x, y, last_card_x, last_card_y) {
                return GameObject::LastCardOfStack(i as u16);
            }

            if let Some(card_i) = get_index_of_card_in_stack_at(&game.stacks, i as u16, x, y) {
                return GameObject::CardOfStack {
                    card_i,
                    stack_i: i as u16,
                };
            }
        }

        GameObject::None
    }

    pub fn get_selected_object(&self) -> GameObject {
        self.selected_object
    }

    pub fn select_object(&mut self, game: &Game, object: GameObject) {
        let can_set = match object {
            GameObject::Pile => !game.pile.is_empty(),
            GameObject::SuitStack(i) => !game.suit_stacks[i as usize].is_empty(),
            GameObject::LastCardOfStack(i) => !game.stacks[i as usize].is_empty(),
            GameObject::None | GameObject::CardOfStack { .. } => true,
            _ => false,
        };

        if can_set {
            self.selected_object = object;
        }
    }

    pub fn set_selected_object_position(&mut self, row: u16, column: u16) {
        self.selected_object_row = row;
        self.selected_object_column = column;
    }

    fn draw_hidden_card(&mut self, x0: u16, y0: u16, color: Color) -> Result<()> {
        let (x, y) = (x0 + PADDING_X, y0 + PADDING_Y);

        self.draw_card_outline(x0, y0, color)?;

        queue!(
            self.out,
            MoveTo(x + 1, y + 1),
            Print("▓▓▓▓▓"),
            MoveTo(x + 1, y + 2),
            Print("▓▓▓▓▓"),
            MoveTo(x + 1, y + 3),
            Print("▓▓▓▓▓"),
        )
    }

    fn draw_card_outline(&mut self, x: u16, y: u16, color: Color) -> Result<()> {
        let (x, y) = (x + PADDING_X, y + PADDING_Y);

        queue!(
            self.out,
            SetForegroundColor(color),
            MoveTo(x, y),
            Print("╔═════╗"),
            MoveTo(x, y + 1),
            Print("║     ║"),
            MoveTo(x, y + 2),
            Print("║     ║"),
            MoveTo(x, y + 3),
            Print("║     ║"),
            MoveTo(x, y + 4),
            Print("╚═════╝"),
        )
    }

    fn draw_card_deshed_outline(&mut self, x: u16, y: u16, color: Color) -> Result<()> {
        let (x, y) = (x + PADDING_X, y + PADDING_Y);

        queue!(
            self.out,
            SetForegroundColor(color),
            MoveTo(x, y),
            Print("╔ ═ ═ ╗"),
            MoveTo(x, y + 1),
            Print("       "),
            MoveTo(x, y + 2),
            Print("║     ║"),
            MoveTo(x, y + 3),
            Print("       "),
            MoveTo(x, y + 4),
            Print("╚ ═ ═ ╝"),
        )
    }

    fn draw_suit_card_placeholder(&mut self, x0: u16, y0: u16, color: Color) -> Result<()> {
        let (x, y) = (x0 + PADDING_X, y0 + PADDING_Y);

        self.draw_card_outline(x0, y0, color)?;

        queue!(self.out, MoveTo(x + 3, y + 2), Print("A"))
    }

    fn draw_card(&mut self, card: &Card, x0: u16, y0: u16, color: Color) -> Result<()> {
        let (x, y) = (x0 + PADDING_X, y0 + PADDING_Y);

        if card.hidden {
            self.draw_hidden_card(x0, y0, color)
        } else {
            self.draw_card_outline(x0, y0, color)?;

            let value = card.value.to_string();
            let suit = card.suit.to_string();

            queue!(
                self.out,
                MoveTo(x + 1, y + 1),
                Print(format!("{value:<2} {suit} ")),
                MoveTo(x + 1, y + 2),
                Print(format!("  {suit}  ")),
                MoveTo(x + 1, y + 3),
                Print(format!(" {suit} {value:>2}")),
            )
        }
    }

    fn draw_cards_in_stack(
        &mut self,
        cards: &[Card],
        x0: u16,
        y0: u16,
        color: Color,
    ) -> Result<()> {
        let cards_len = cards.len() as u16;

        for i in 0..cards_len {
            let card = &cards[i as usize];
            let x = x0 + i * STACK_CARD_X_OFFSET;
            let y = y0 + i * STACK_CARD_Y_OFFSET;
            self.draw_card(card, x, y, color)?;
        }

        Ok(())
    }

    fn render_pile(&mut self, pile: &Pile) -> Result<()> {
        let (mut x, y) = get_pile_position(pile);

        let visible_cards = pile.get_visible_cards();
        let all_cards_count = pile.len() as u16;
        let mut visible_cards_count = visible_cards.len() as u16;

        if visible_cards_count < all_cards_count {
            self.draw_card_outline(x, y, Color::White)?;
            x += 1;
        }

        if let GameObject::Pile = self.selected_object {
            visible_cards_count -= 1;
        }

        for i in 0..visible_cards_count {
            let card = &visible_cards[i as usize];
            self.draw_card(card, x, y, Color::White)?;
            x += CARD_COLS.div(2);
        }

        Ok(())
    }

    fn render_deck(&mut self, deck: &Deck) -> Result<()> {
        let (x, y) = get_deck_position(deck);

        match deck.len() {
            0 => self.draw_card_outline(x, y, Color::DarkGrey),
            1 => self.draw_hidden_card(x, y, Color::White),
            _ => {
                self.draw_hidden_card(x + 1, y, Color::White)?;
                self.draw_hidden_card(x, y, Color::White)
            }
        }
    }

    fn render_suit_stack(&mut self, stack: &Stack, i: u16) -> Result<()> {
        let (x, y) = get_suit_stack_position(i);

        let last_card = match self.selected_object {
            GameObject::SuitStack(j) if j == i => stack.prelast(),
            _ => stack.last(),
        };

        match last_card {
            Some(card) => self.draw_card(card, x, y, Color::White),
            None => self.draw_suit_card_placeholder(x, y, Color::DarkGrey),
        }
    }

    fn render_stack(&mut self, stack: &Stack, i: u16) -> Result<()> {
        let (x, y) = get_stack_position(i);

        self.draw_card_deshed_outline(x, y, Color::DarkGrey)?;

        let cards = stack.get_all();
        let len = match self.selected_object {
            GameObject::CardOfStack { card_i, stack_i } if stack_i == i => card_i as usize,
            GameObject::LastCardOfStack(stack_i) if stack_i == i => cards.len() - 1,
            _ => cards.len(),
        };

        self.draw_cards_in_stack(&cards[..len], x, y, Color::White)
    }

    fn render_selected_object(&mut self, game: &Game) -> Result<()> {
        let object = self.selected_object;
        let (x, y) = (self.selected_object_column, self.selected_object_row);

        let selected_card = match object {
            GameObject::Pile => game.pile.get_visible_cards().last(),
            GameObject::SuitStack(i) => game.suit_stacks[i as usize].last(),
            GameObject::LastCardOfStack(i) => game.stacks[i as usize].last(),
            _ => None,
        };

        if let Some(card) = selected_card {
            self.draw_card(card, x, y, Color::White)?;
        } else {
            if let GameObject::CardOfStack { card_i, stack_i } = object {
                let stack = &game.stacks[stack_i as usize];
                let cards = &stack.get_all()[card_i as usize..];
                self.draw_cards_in_stack(cards, x, y, Color::White)?;
            }
        }

        Ok(())
    }
}

fn get_deck_position(deck: &Deck) -> (u16, u16) {
    let x = EMPTY_DECK_X;
    let y = EMPTY_DECK_Y;

    if deck.len() <= 1 {
        (x, y)
    } else {
        (x - 1, y)
    }
}

fn get_pile_position(pile: &Pile) -> (u16, u16) {
    let mut x = PILE_LAST_CARD_X;
    let y = PILE_LAST_CARD_Y;

    let all_cards_count = pile.len() as u16;
    let visible_cards_count = pile.get_visible_cards().len() as u16;

    x -= visible_cards_count.checked_sub(1).unwrap_or(0) * CARD_COLS.div(2);

    if visible_cards_count < all_cards_count {
        x -= 1;
    }

    (x, y)
}

fn get_suit_stack_position(i: u16) -> (u16, u16) {
    let x = FIRST_SUIT_STACK_X + i * (CARD_COLS + GAP_BETWEEN_STACK_CARDS);
    let y = FIRST_SUIT_STACK_Y;

    (x, y)
}

fn get_stack_position(i: u16) -> (u16, u16) {
    let x = FIRST_STACK_X + i * (CARD_COLS + GAP_BETWEEN_STACK_CARDS);
    let y = FIRST_STACK_Y;

    (x, y)
}

fn get_stack_last_card_position(stacks: &[Stack], i: u16) -> (u16, u16) {
    let (mut x, mut y) = get_stack_position(i);

    let cards_count = stacks[i as usize].len() as u16;
    let offsets_count = cards_count.checked_sub(1).unwrap_or(0);

    x += offsets_count * STACK_CARD_X_OFFSET;
    y += offsets_count * STACK_CARD_Y_OFFSET;

    (x, y)
}

fn get_index_of_card_in_stack_at(stacks: &[Stack], stack_i: u16, x: u16, y: u16) -> Option<u16> {
    let (mut card_x, mut card_y) = get_stack_last_card_position(stacks, stack_i);

    let cards = stacks[stack_i as usize].get_all();

    for (i, card) in cards.iter().enumerate().rev() {
        if card.hidden {
            return None;
        }

        if is_point_inside_card(x, y, card_x, card_y) {
            return Some(i as u16);
        }

        if i > 0 {
            card_x -= STACK_CARD_X_OFFSET;
            card_y -= STACK_CARD_Y_OFFSET;
        }
    }

    None
}

fn is_point_outside_of_table(x: u16, y: u16) -> bool {
    x < PADDING_X || y < PADDING_Y || x >= PADDING_X + TABLE_COLS || y >= PADDING_Y + TABLE_ROWS
}

fn is_point_inside_card(px: u16, py: u16, cx: u16, cy: u16) -> bool {
    px >= cx && px < cx + CARD_COLS && py >= cy && py < cy + CARD_ROWS
}
