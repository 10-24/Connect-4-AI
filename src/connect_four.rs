use rayon::prelude::*;
use std::cmp::{max, min};

pub struct ConnectFour {
    pub board: [[Token; 6]; 7],
    pub column_height: [i8; 7],
    pub player: Player,
}
impl ConnectFour {
    pub fn new() -> ConnectFour {
        ConnectFour {
            board: [[Token::Empty; 6]; 7],
            column_height: [0; 7],
            player: Player::Blue,
        }
    }

    pub fn play(&mut self, selected_x: i8) -> bool {
        /*Remember to swap players after calling */

        let new_token = Point {
            x: selected_x,
            y: self.column_height[selected_x as usize],
        };

        self.board[new_token.x as usize][new_token.y as usize] = Token::Base(self.player);
        self.column_height[new_token.x as usize] += 1;
        self.player_won(new_token)
    }
    pub fn swap_players(&mut self) {
        self.player = match self.player {
            Player::Blue => Player::Red,
            Player::Red => Player::Blue,
        };
    }

    fn player_won(&self, new_token: Point<i8>) -> bool {
        let directions = [
            // CheckDirection::North,
            // CheckDirection::East, 
            // CheckDirection::NorthEast,
            CheckDirection::SouthEast,
        ];
        directions
            .par_iter()
            .any(|delta| self.check_for_win(&new_token, delta))
    }

    fn check_for_win(&self, new_token_pos: &Point<i8>, check_direction: &CheckDirection) -> bool {
        fn get_starting_x(x_pos: &i8, check_direction: &CheckDirection) -> i8 {
            match check_direction {
                CheckDirection::North => x_pos.to_owned(),
                CheckDirection::East => max(x_pos - 3, 0), // !
                CheckDirection::NorthEast => max(x_pos - 3, 0), // !
                CheckDirection::SouthEast => max(x_pos - 3, 0),
            }
        }
        fn get_starting_y(y_pos: &i8, check_direction: &CheckDirection) -> i8 {
            match check_direction {
                CheckDirection::North => max(y_pos - 3, 0),
                CheckDirection::East => y_pos.to_owned(),
                CheckDirection::NorthEast => max(y_pos - 3, 0),
                CheckDirection::SouthEast => min(y_pos + 3, 5),
            }
        }

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
        fn calculate_max_checks(new_token_pos: &Point<i8>, check_direction: &CheckDirection) -> i8 {
            fn number_of_x_checks(x: &i8) -> i8 {
                -(x - 3).abs() + 7
            }
            fn number_of_y_checks(y: &i8) -> i8 {
                let checks = -((*y as f32) - 2.5).abs() + 5.5;
                checks as i8
            }
            match check_direction {
                CheckDirection::North => number_of_y_checks(&new_token_pos.y),
                CheckDirection::East => number_of_x_checks(&new_token_pos.x),
                CheckDirection::NorthEast => min(
                    number_of_x_checks(&new_token_pos.x),
                    number_of_y_checks(&new_token_pos.y),
                ),
                CheckDirection::SouthEast => min(
                    number_of_x_checks(&new_token_pos.x),
                    number_of_y_checks(&new_token_pos.y),
                ),
            }
        }
        let mut point: Point<i8> = Point {
            x: get_starting_x(&new_token_pos.x, check_direction),
            y: get_starting_y(&new_token_pos.y, check_direction),
        };

        let increments = convert_direction_to_demension_increments(check_direction);

        let max_checks = calculate_max_checks(new_token_pos, check_direction);

        
        println!("new_token {:?}", new_token_pos);
        println!("max_checks: {max_checks}");
        println!("Start {:?}", point);
        let mut series_len: i8 = 0;
        for _ in 0..max_checks {
            if let Token::Base(player) = self.board[point.x as usize][point.y as usize] {
                if self.player == player {
                    series_len += 1;
                    if series_len == 4 {
                        return true;
                    }
                } else {
                    series_len = 0; // Clear
                }
            }
            point.x += increments.x;
            point.y += increments.y;
        }
        println!("End {:?}", point);
        false
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
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

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Base(Player),
    Empty,
}
