use candle_core::{Device, Tensor};
use nalgebra::Point2;
use std::{fmt, ops::Neg};

use crate::player::Player;

use super::connect_four_enums::GameOutcome;
pub struct ConnectFour {
   
    pub board: [f32; 42],
    pub column_height: [usize; 7],
    pub current_player: Player,
    pub victory_threshold: u8,
    pub tokens_placed: u8,
}
pub type GameBoard = [f32; 42];\
impl ConnectFour {
    
    pub const COLS: usize = 7;
    pub const ROWS: usize = 6;

    pub const TOTAL_SPACES: usize = Self::COLS * Self::ROWS;
  
    pub fn new(victory_threshold: u8) -> ConnectFour {
        ConnectFour {
            victory_threshold,
            board: [0.0; Self::COLS * Self::ROWS],
            column_height: [0; 7],
            current_player: Player::Blue,
            tokens_placed: 0,
        }
    }

    pub fn reset(&mut self) {
        self.board.fill(0.0);
        self.column_height.fill(0);
        self.current_player = Player::Blue;
        self.tokens_placed = 0;
    }

    pub fn play_turn(&mut self, selected_col: usize) -> Option<GameOutcome> {
        let selected_row = self.column_height[selected_col];

        if selected_row >= Self::ROWS {
            self.current_player.switch();
            return None;
        }

        let new_token_pos = Point2::new(selected_col, selected_row);
        self.set_to_current_players_token(&new_token_pos);

        self.column_height[new_token_pos.x] += 1;
        self.tokens_placed += 1;

        if self.player_won(new_token_pos) {
            return Some(GameOutcome::Win);
        }

        if self.tokens_placed >= Self::TOTAL_SPACES as u8 {
            return Some(GameOutcome::Tie);
        }

        self.current_player.switch();
        None
    }

    fn point_to_index(point: &Point2<usize>) -> usize {
        point.y * Self::COLS + point.x
    }

    fn set_to_current_players_token(&mut self, point: &Point2<usize>) {
        let index = Self::point_to_index(point);
        self.board[index] = self.current_player.to_f32();
    }

    pub fn get(&self, point: &Point2<usize>) -> &f32 {
        &self.board[Self::point_to_index(point)]
    }
    pub fn get_token(&self, point: &Point2<usize>) -> Option<Player> {
        let val = self.get(point);
        Player::from_f32(val)
    }

    pub fn get_board_blue_perspective(&self) -> [f32;ConnectFour::TOTAL_SPACES] {
        self.board.clone()
    }
    pub fn get_board_red_perspective(&self) -> [f32;ConnectFour::TOTAL_SPACES] {
        let mut cloned_board = self.board.clone();
        for i in cloned_board.iter_mut() {
            *i *= -1.0;
        }
        cloned_board
    }
    fn board_to_tensor(){

    }
}

