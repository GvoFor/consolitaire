use super::card::Card;
use std::fmt;

#[derive(Debug)]
pub struct Stack(Vec<Card>);

struct StackSlice<'a>(&'a [Card]);

#[derive(Debug)]
pub struct Pile {
    cards: Stack,
    size: usize,
}

#[derive(Debug)]
pub struct Deck {
    cards: Stack,
}

impl Stack {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn last(&self) -> Option<&Card> {
        self.0.last()
    }

    pub fn pop_into(&mut self, stack: &mut Stack) {
        if let Some(card) = self.0.pop() {
            stack.push(card);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn reveal_all(&mut self) {
        self.0.iter_mut().for_each(|card| card.reveal());
    }

    pub fn reveal_last(&mut self) {
        if let Some(last) = self.0.last_mut() {
            last.reveal();
        }
    }

    pub fn get_all(&self) -> &[Card] {
        &self.0
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let items = self
            .0
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(formatter, "{items}")
    }
}

impl<'a> fmt::Display for StackSlice<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items = self
            .0
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join(", ");
        write!(formatter, "{items}")
    }
}

impl Pile {
    pub fn new(size: usize, capacity: usize) -> Self {
        Self {
            cards: Stack::with_capacity(capacity),
            size,
        }
    }

    pub fn pull_from(&mut self, deck: &mut Deck) {
        for _ in 0..self.size {
            if let Some(card) = deck.pop() {
                self.cards.push(card);
            }
        }
    }

    pub fn pop_all_into(&mut self, deck: &mut Deck) {
        while let Some(card) = self.cards.pop() {
            deck.push(card);
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn get_visible_cards(&self) -> &[Card] {
        let end = self.len();
        let start = end.checked_sub(self.size).unwrap_or(0);
        &self.cards.get_all()[start..end]
    }

    pub fn get_cards_mut(&mut self) -> &mut Stack {
        &mut self.cards
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }
}

impl fmt::Display for Pile {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let len = self.len();
        let nonvisible_cards = if len > self.size { "▓, " } else { "" };
        let visible_cards = StackSlice(self.get_visible_cards());
        write!(formatter, "{nonvisible_cards}{visible_cards} ({len} cards)")
    }
}

impl Deck {
    pub fn random() -> Self {
        use super::card::{Suit::*, Value::*};
        use itertools::iproduct;
        use rand::{seq::SliceRandom, thread_rng};

        let mut cards: Vec<Card> = iproduct!(
            [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace],
            [Clubs, Hearts, Spades, Diamonds]
        )
        .map(|(value, suit)| Card::new(value, suit))
        .collect();

        cards.shuffle(&mut thread_rng());

        Deck {
            cards: Stack(cards),
        }
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn pop_into(&mut self, stack: &mut Stack) {
        self.cards.pop_into(stack);
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn reveal_all(&mut self) {
        self.cards.reveal_all();
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let len = self.len();
        let deck: &str = if len > 0 { "▓" } else { "" };
        write!(formatter, "{deck} ({len} cards)")
    }
}
