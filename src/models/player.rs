#[derive(Clone, PartialEq)]

pub enum Player {
    X,
    O,
}

impl Player {
    pub fn char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    pub fn other(&self) -> char {
        match self {
            Player::X => 'O',
            Player::O => 'X',
        }
    }
}