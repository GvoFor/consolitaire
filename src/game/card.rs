use std::fmt;

#[derive(PartialEq)]
enum Color {
    Black,
    Red,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Debug)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub hidden: bool,
}

pub enum CoveringOrder {
    Ascending,
    Descending,
}

impl Suit {
    fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Spades | Suit::Clubs => Color::Black,
        }
    }

    fn has_same_color_with(&self, another: &Self) -> bool {
        self.color() == another.color()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let suit = match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
        };
        write!(formatter, "{suit}")
    }
}

impl Value {
    fn is_next_after(&self, value: &Self) -> bool {
        u8::from(self) == u8::from(value) + 1
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        };
        write!(formatter, "{value}")
    }
}

impl From<&Value> for u8 {
    fn from(value: &Value) -> Self {
        match value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 1,
        }
    }
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Card {
            value,
            suit,
            hidden: true,
        }
    }

    pub fn can_one_be_covered_with_another(
        card_to_cover: Option<&Card>,
        covering_card: Option<&Card>,
        order: CoveringOrder,
    ) -> bool {
        match (card_to_cover, covering_card) {
            (Some(card_to_cover), Some(covering_card)) => {
                card_to_cover.can_be_covered_with(covering_card, order)
            }
            (None, Some(covering_card)) => match order {
                CoveringOrder::Ascending => covering_card.value == Value::Ace,
                CoveringOrder::Descending => covering_card.value == Value::King,
            },
            _ => false,
        }
    }

    fn can_be_covered_with(&self, card: &Card, order: CoveringOrder) -> bool {
        match order {
            CoveringOrder::Ascending => {
                self.suit == card.suit && card.value.is_next_after(&self.value)
            }
            CoveringOrder::Descending => {
                !self.suit.has_same_color_with(&card.suit) && self.value.is_next_after(&card.value)
            }
        }
    }

    pub fn reveal(&mut self) {
        self.hidden = false;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let value = &self.value;
        let suit = &self.suit;
        if self.hidden {
            write!(formatter, "▓")
        } else {
            write!(formatter, "[{value}{suit}]")
        }
    }
}
