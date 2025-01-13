use rayon::prelude::*;
use std::{fmt, ops::Neg};

pub struct ConnectFour {
    pub board: [[Token; 6]; 7],
    pub column_height: [i8; 7],
    pub current_player: Player,
}
impl ConnectFour {
    pub fn new() -> ConnectFour {
        ConnectFour {
            board: [[Token::Empty; 6]; 7],
            column_height: [0; 7],
            current_player: Player::Blue,
        }
    }

    pub fn play(&mut self, selected_x: i8) -> bool {
        /*Remember to swap players after calling */

        let new_token = Point {
            x: selected_x,
            y: self.column_height[selected_x as usize],
        };

        self.board[new_token.x as usize][new_token.y as usize] = Token::Base(self.current_player);
        self.column_height[new_token.x as usize] += 1;
        self.player_won(new_token)
    }
    pub fn swap_players(&mut self) {
        self.current_player = match self.current_player {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        };
    }

    fn player_won(&self, new_token: Point<i8>) -> bool {
        let directions = [
            CheckDirection::North,
            CheckDirection::East,
            CheckDirection::NorthEast,
            CheckDirection::SouthEast,
        ];
        directions
            .par_iter()
            .any(|delta| self.check_for_win(&new_token, delta))
    }

    fn check_for_win(&self, new_token_pos: &Point<i8>, check_direction: &CheckDirection) -> bool {
        fn convert_direction_to_demension_increments(
            check_direction: &CheckDirection,
        ) -> Point<i8> {
            match check_direction {
                CheckDirection::North => Point { x: 0, y: 1 },
                CheckDirection::East => Point { x: 1, y: 0 },
                CheckDirection::NorthEast => Point { x: 1, y: 1 },
                CheckDirection::SouthEast => Point { x: 1, y: -1 },
            }
        }

        let mut increments = convert_direction_to_demension_increments(check_direction);

        let mut series_len:u8 = 1; // 1 to account for the new token 
        series_len += self.march_in_direction(new_token_pos,&increments);
        increments.negate();
        series_len += self.march_in_direction(new_token_pos,&increments);
        
        series_len >= 4
    }
    fn march_in_direction(&self, new_token_pos: &Point<i8>, increments: &Point<i8>) -> u8 {
        
     
        let mut pos = new_token_pos.clone();
        let mut series_len = 0 ;

        loop {
            pos.x += increments.x;
            pos.y += increments.y;

            // Checking for underflow
            if pos.x < 0 || pos.y < 0 {
                return series_len;
            }
            // Checking for overflow
            if pos.x as usize >= self.board.len() || pos.y as usize >= self.board[0].len() {
                return series_len;
            }

            if let Token::Base(token_owner) = self.board[pos.x as usize][pos.y as usize] {
                if token_owner != self.current_player {
                    return series_len;
                }
                series_len += 1;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Point<T>
where
    T: Neg<Output = T> + Copy, // Constraint for signed types
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Neg<Output = T> + Copy,
{
    fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
}
enum CheckDirection {
    North,
    East,
    NorthEast,
    SouthEast,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Blue,
    Red,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Blue => write!(f, "Blue"),
            Player::Red => write!(f, "Red"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Base(Player),
    Empty,
}
