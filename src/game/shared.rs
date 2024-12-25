pub const PADDING_X: u16 = 2;
pub const PADDING_Y: u16 = 1;
pub const TABLE_COLS: u16 = 78;
pub const TABLE_ROWS: u16 = 35;

#[derive(Debug, Clone, Copy)]
pub enum GameObject {
    Deck,
    Pile,
    LastCardOfStack(u16),
    SuitStack(u16),
    None,
}

impl GameObject {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}
