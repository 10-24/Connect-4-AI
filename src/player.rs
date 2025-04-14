use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Blue,
    Red,
}

impl Player {
    pub fn to_val(&self) -> f32 {
        match self {
            Player::Blue => 1.0,
            Player::Red => -1.0,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Player::Blue => "Blue".to_string(),
            Player::Red => "Red".to_string(),
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
    pub fn from_val(val: f32) -> Option<Player> {
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
