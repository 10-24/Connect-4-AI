use nalgebra::Point2;
use std::{fmt, ops::Neg};

use crate::player::Player;

use super::{connect_four_enums::Outcome, game_board::{self, GameBoard}};
pub struct ConnectFour {
    pub board: GameBoard,
    pub current_player: Player,
    pub victory_threshold: u8,
    pub tokens_placed: u8,
}

impl ConnectFour {
    pub fn new(victory_threshold: u8) -> ConnectFour {
        ConnectFour {
            victory_threshold,
            board: GameBoard::new(),
            current_player: Player::Blue,
            tokens_placed: 0,
        }
    }

    pub fn reset(&mut self) {
        self.current_player = Player::Blue;
        self.tokens_placed = 0;
    }

    pub fn play_turn(&mut self, col: usize) -> Option<Outcome> {

        let new_token_pos = self.board.add_token(col, self.current_player);
        if let Some(new_token_pos) = new_token_pos {

    
            if self.player_won(new_token_pos) {
                return Some(Outcome::Win(self.current_player.clone()));
            }

            self.tokens_placed += 1;

            if self.tokens_placed >= GameBoard::TOTAL_SPACES as u8 {
                return Some(Outcome::Tie);
            }

            None
        } else {
            None
        }
    }
}
