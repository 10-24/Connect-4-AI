use std::ops::Neg;

#[derive(PartialEq)]
pub enum GameOutcome {
    Win,
    Tie,
    Loss,
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

