pub mod card;
pub mod card_collections;
pub mod engine;
pub mod renderer;

use card::{Card, CoveringOrder};
use card_collections::{Deck, Pile, Stack};

const CARDS_IN_SUIT: usize = 13;
const CARDS_IN_DECK: usize = CARDS_IN_SUIT * 4;

#[derive(Debug)]
pub struct Game {
    deck: Deck,
    suit_stacks: [Stack; 4],
    stacks: Vec<Stack>,
    pile: Pile,
}

impl Game {
    pub fn new(stacks_count: usize, pile_size: usize) -> Self {
        let pile = Pile::new(pile_size, CARDS_IN_DECK);
        let suit_stacks = [
            Stack::with_capacity(CARDS_IN_SUIT),
            Stack::with_capacity(CARDS_IN_SUIT),
            Stack::with_capacity(CARDS_IN_SUIT),
            Stack::with_capacity(CARDS_IN_SUIT),
        ];

        let mut deck = Self::generate_deck();
        let mut stacks: Vec<Stack> = (0..stacks_count)
            .map(|_| Stack::with_capacity(CARDS_IN_SUIT))
            .collect();

        Self::rearange_cards(&mut deck, &mut stacks);

        Self {
            deck,
            suit_stacks,
            stacks,
            pile,
        }
    }

    fn generate_deck() -> Deck {
        Deck::random()
    }

    fn rearange_cards(deck: &mut Deck, stacks: &mut Vec<Stack>) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            for _ in 0..i {
                deck.pop_into(stack);
            }
        }

        deck.reveal_all();

        for stack in stacks {
            stack.reveal_last();
        }
    }

    fn get_deck(&self) -> &Deck {
        &self.deck
    }

    fn get_suit_stacks(&self) -> &[Stack; 4] {
        &self.suit_stacks
    }

    fn get_stacks(&self) -> &[Stack] {
        &self.stacks
    }

    fn get_pile(&self) -> &Pile {
        &self.pile
    }

    fn restart(&mut self) {
        self.stacks.iter_mut().for_each(|stack| stack.clear());
        self.suit_stacks.iter_mut().for_each(|stack| stack.clear());
        self.pile.clear();

        let mut deck = Self::generate_deck();
        Self::rearange_cards(&mut deck, &mut self.stacks);

        self.deck = deck;
    }

    fn move_cards_from_deck_to_pile(&mut self) {
        if self.deck.is_empty() {
            self.pile.pop_all_into(&mut self.deck);
        } else {
            self.pile.pull_from(&mut self.deck);
        }
    }

    fn pop_card_from_stack_into_stack(
        from: &mut Stack,
        into: &mut Stack,
        order: CoveringOrder,
    ) -> bool {
        let covering_card = from.last();
        let card_to_cover = into.last();

        if Card::can_one_be_covered_with_another(card_to_cover, covering_card, order) {
            from.pop_into(into);
            from.reveal_last();
            true
        } else {
            false
        }
    }

    fn move_card_from_pile_to_stack(&mut self, i: usize) -> bool {
        let from = self.pile.get_cards_mut();
        if let Some(into) = self.stacks.get_mut(i) {
            Self::pop_card_from_stack_into_stack(from, into, CoveringOrder::Descending)
        } else {
            false
        }
    }

    fn move_card_from_pile_to_suit_stack(&mut self, i: usize) -> bool {
        let from = self.pile.get_cards_mut();
        if let Some(into) = self.suit_stacks.get_mut(i) {
            Self::pop_card_from_stack_into_stack(from, into, CoveringOrder::Ascending)
        } else {
            false
        }
    }

    fn move_card_from_stack_to_stack(&mut self, i: usize, j: usize) -> bool {
        if i == j || i >= self.stacks.len() || j >= self.stacks.len() {
            return false;
        }

        let (left, right) = self.stacks.split_at_mut(j.max(i));

        let (from, into) = if i < j {
            (&mut left[i], &mut right[0])
        } else {
            (&mut right[0], &mut left[j])
        };

        Self::pop_card_from_stack_into_stack(from, into, CoveringOrder::Descending)
    }

    fn move_card_from_stack_to_suit_stack(&mut self, i: usize, j: usize) -> bool {
        if let Some(from) = self.stacks.get_mut(i) {
            if let Some(into) = self.suit_stacks.get_mut(j) {
                return Self::pop_card_from_stack_into_stack(from, into, CoveringOrder::Ascending);
            }
        }
        false
    }

    fn move_card_from_suit_stack_to_stack(&mut self, i: usize, j: usize) -> bool {
        if let Some(from) = self.suit_stacks.get_mut(i) {
            if let Some(into) = self.stacks.get_mut(j) {
                return Self::pop_card_from_stack_into_stack(from, into, CoveringOrder::Descending);
            }
        }
        false
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new(7, 3)
    }
}
