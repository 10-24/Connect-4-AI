use std::ops::Neg;



use super::player::{self, Player};

#[derive(PartialEq,Clone,Copy)]
pub enum Outcome {
    Win(Player),
    Tie,
}
impl Outcome {
    pub fn reward(&self,player:&Player) -> f32 {
        if let Self::Win(victorious_player) = *self {
            if *player == victorious_player {
                return 0.5;
            }
            return -0.4;
        }

        -0.08
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Delta {
    Increment,
    Zero,
    Decrement,
}
impl Neg for Delta {
    type Output = Delta;

    fn neg(self) -> Self::Output {
        match self {
            Delta::Increment => Delta::Decrement,
            Delta::Zero => Delta::Zero,
            Delta::Decrement => Delta::Increment,
        }
    }
}

impl Delta {
    pub fn to_i8(&self) -> i8 {
        match &self {
            Delta::Increment => 1,
            Delta::Zero => 0,
            Delta::Decrement => -1,
        }
    }
}

