use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Blue,
    Red,
}

impl Player {
    pub fn to_val(&self) -> f32 {
        match self {
            Player::Blue => 1,
            Player::Red => -1,
        }
    }
}

impl Player {
    pub fn switch(&mut self) {
        *self = match self {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        };
    }
    pub fn from_val(val: i8) -> Option<Player> {
        match val {
            1 => Some(Player::Blue),
            -1 => Some(Player::Red),
            _ => None,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Blue => write!(f, "Blue"),
            Player::Red => write!(f, "Red"),
        }
    }
}
