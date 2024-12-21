use super::Game;

pub struct TextOutputRenderer;

impl TextOutputRenderer {
    pub fn render(game: &Game) {
        let deck = game.get_deck();
        println!("Deck: {deck}");
        
        let pile = game.get_pile();
        println!("Pile: {pile}");
        
        println!("");
        
        let suit_stacks = game.get_suit_stacks();
        for (i, stack) in suit_stacks.iter().enumerate() {
            println!("Suit stack {}: {stack}", i + 1);
        }
        
        println!("");
        
        let stacks = game.get_stacks();
        for (i, stack) in stacks.iter().enumerate() {
            println!("Suit stack {}: {stack}", i + 1);
        }
    }
}