use super::card::Card;

#[derive(Debug)]
pub struct Stack(Vec<Card>);

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

    pub fn prelast(&self) -> Option<&Card> {
        if self.0.len() < 2 {
            None
        } else {
            self.0.get(self.0.len() - 2)
        }
    }

    pub fn pop_into(&mut self, stack: &mut Stack) {
        if let Some(card) = self.0.pop() {
            stack.push(card);
        }
    }

    pub fn pop_n_last_into(&mut self, n: usize, stack: &mut Stack) {
        for _ in 0..n {
            if let Some(card) = self.0.pop() {
                stack.push(card);
            } else {
                break;
            }
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

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
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
