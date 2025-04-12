use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Blue,
    Red,
}

impl Player {
    pub fn to_f32(&self) -> f32 {
        match self {
            Player::Blue => 1.0,
            Player::Red => -1.0,
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
    pub fn from_f32(val: &f32) -> Option<Player> {
        match val {
            1.0 => Some(Player::Blue),
            -1.0 => Some(Player::Red),
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
