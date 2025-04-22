
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug,Serialize,Deserialize,Default)]
pub enum Player {
    #[default]
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
}

impl Player {
    pub fn switch(&mut self) {
        *self = self.other_player();
    }
    pub fn other_player(&self) -> Self {
        match &self {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        }
    }
    pub fn from_val(val: f32) -> Option<Player> {
        match val {
            1.0 => Some(Player::Blue),
            -1.0 => Some(Player::Red),
            _ => None,
        }
    }
}

