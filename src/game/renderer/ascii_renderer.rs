use super::Renderer;
use crate::game::{card::Card, Game};

use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType, SetSize},
};
use std::io::{stdout, Result, Stdout};

const PADDING_X: u16 = 2;
const PADDING_Y: u16 = 1;
const TABLE_COLS: u16 = 78;
const TABLE_ROWS: u16 = 35;
const CARD_COLS: u16 = 7;
const CARD_ROWS: u16 = 5;
const GAP_BETWEEN_SUIT_CARDS: u16 = 3;
const GAP_BETWEEN_PILE_AND_DECK: u16 = 6;

pub struct ASCIIRenderer;

impl Renderer<Game> for ASCIIRenderer {
    fn render(game: &Game) -> Result<()> {
        let mut out = stdout();

        execute!(
            out,
            Clear(ClearType::All),
            SetSize(TABLE_COLS + PADDING_X * 2, TABLE_ROWS + PADDING_Y * 2),
        )?;

        let deck = &game.deck;
        let stacks = &game.stacks;
        let suit_stacks = &game.suit_stacks;
        let pile = &game.pile;

        // Render suit stacks
        for i in 0..suit_stacks.len() {
            let x = i as u16 * (CARD_COLS + GAP_BETWEEN_SUIT_CARDS);
            let y = 0;

            match suit_stacks[i].last() {
                Some(card) => Self::draw_card(&mut out, card, x, y, Color::White)?,
                None => Self::draw_suit_card_placeholder(&mut out, x, y, Color::DarkGrey)?,
            }
        }

        // Render pile
        {
            let mut x = TABLE_COLS - CARD_COLS - 1 - GAP_BETWEEN_PILE_AND_DECK;
            let y = 0;

            let pile_len = pile.len() as u16;
            x -= pile_len.min(1) * CARD_COLS;
            x -= pile_len.checked_sub(1).unwrap_or(0).min(3) * 3;

            let visible_cards = pile.get_visible_cards();

            if pile_len > visible_cards.len() as u16 {
                x += 3;
                Self::draw_card_outline(&mut out, x - 1, y, Color::White)?;
            }

            for card in visible_cards {
                Self::draw_card(&mut out, card, x, y, Color::White)?;
                x += 3;
            }
        }

        // Render deck
        {
            let x = TABLE_COLS - CARD_COLS;
            let y = 0;

            if deck.is_empty() {
                Self::draw_card_outline(&mut out, x, y, Color::DarkGrey)?;
            } else {
                Self::draw_hidden_card(&mut out, x, y, Color::White)?;

                if deck.len() > 1 {
                    Self::draw_hidden_card(&mut out, x - 1, y, Color::White)?;
                }
            }
        }

        // Render stacks
        for i in 0..stacks.len() {
            let mut x = i as u16 * (CARD_COLS + GAP_BETWEEN_SUIT_CARDS);
            let mut y = CARD_ROWS + 1;

            Self::draw_card_deshed_outline(&mut out, x, y, Color::DarkGrey)?;

            for card in stacks[i].get_all() {
                Self::draw_card(&mut out, card, x, y, Color::White)?;
                x += 1;
                y += 2;
            }
        }

        execute!(
            out,
            SetForegroundColor(Color::White),
            MoveTo(PADDING_X, PADDING_Y + TABLE_ROWS),
        )?;

        Ok(())
    }
}

impl ASCIIRenderer {
    fn draw_hidden_card(out: &mut Stdout, x0: u16, y0: u16, color: Color) -> Result<()> {
        let (x, y) = (x0 + PADDING_X, y0 + PADDING_Y);

        Self::draw_card_outline(out, x0, y0, color)?;

        queue!(
            out,
            MoveTo(x + 1, y + 1),
            Print("▓▓▓▓▓"),
            MoveTo(x + 1, y + 2),
            Print("▓▓▓▓▓"),
            MoveTo(x + 1, y + 3),
            Print("▓▓▓▓▓"),
        )
    }

    fn draw_card_outline(out: &mut Stdout, x: u16, y: u16, color: Color) -> Result<()> {
        let (x, y) = (x + PADDING_X, y + PADDING_Y);

        queue!(
            out,
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

    fn draw_card_deshed_outline(out: &mut Stdout, x: u16, y: u16, color: Color) -> Result<()> {
        let (x, y) = (x + PADDING_X, y + PADDING_Y);

        queue!(
            out,
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

    fn draw_suit_card_placeholder(out: &mut Stdout, x0: u16, y0: u16, color: Color) -> Result<()> {
        let (x, y) = (x0 + PADDING_X, y0 + PADDING_Y);

        Self::draw_card_outline(out, x0, y0, color)?;

        queue!(out, MoveTo(x + 3, y + 2), Print("A"))
    }

    fn draw_card(out: &mut Stdout, card: &Card, x0: u16, y0: u16, color: Color) -> Result<()> {
        let (x, y) = (x0 + PADDING_X, y0 + PADDING_Y);

        if card.hidden {
            Self::draw_hidden_card(out, x0, y0, color)
        } else {
            Self::draw_card_outline(out, x0, y0, color)?;

            let value = card.value.to_string();
            let suit = card.suit.to_string();

            queue!(
                out,
                MoveTo(x + 1, y + 1),
                Print(format!("{value:<2} {suit} ")),
                MoveTo(x + 1, y + 2),
                Print(format!("  {suit}  ")),
                MoveTo(x + 1, y + 3),
                Print(format!(" {suit} {value:>2}")),
            )
        }
    }
}
