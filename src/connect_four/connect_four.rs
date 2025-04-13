use candle_core::{Device, Tensor};
use nalgebra::Point2;
use std::{fmt, ops::Neg};

use crate::player::Player;

use super::{connect_four_enums::GameOutcome, game_board::GameBoard};
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

    pub fn play_turn(&mut self, selected_col: usize) -> Option<GameOutcome> {
        

        let placed_token = self.board.place_token(selected_col, self.current_player);
        if placed_token == None {
            return Some(GameOutcome::Tie);
        } else {
            let Some(new_token_pos) = placed_token;
            self.tokens_placed += 1;
    
            if self.player_won(new_token_pos) {
                return Some(GameOutcome::Win);
            }
    
            if self.tokens_placed >= GameBoard::TOTAL_SPACES as u8 {
                return Some(GameOutcome::Tie);
            }
    
            self.current_player.switch();
            None
        }
       
    }

    pub fn get_token(&self, point: &Point2<usize>) -> Option<Player> {
        let val = self.get(point);
        Player::from_val(val)
    }

    pub fn get_board_blue_perspective(&self, device: &Device) -> Tensor {
        Self::board_to_tensor(self.board, device)
    }
    pub fn get_board_red_perspective(&self, device: &Device) -> Tensor {
        let mut cloned_board = self.board;
        for i in cloned_board.iter_mut() {
            *i *= -1.0;
        }
        Self::board_to_tensor(cloned_board, device)
    }
    fn board_to_tensor(board: GameBoard, device: &Device) -> Tensor {
        Tensor::from_slice(board.as_slice(), (1, Self::TOTAL_SPACES), device).unwrap()
    }
}
