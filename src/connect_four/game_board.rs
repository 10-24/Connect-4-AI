use crate::{player::Player, Bknd, DEVICE};
use burn::tensor::Tensor;
use nalgebra::Point2;

#[derive(Clone)]
pub struct GameBoard {
    board: [f32; Self::TOTAL_SPACES],

    pub column_height: [usize; Self::COLS],
}
impl GameBoard {
    pub const COLS: usize = 7;
    pub const ROWS: usize = 6;
    pub const TOTAL_SPACES: usize = Self::COLS * Self::ROWS;

    pub fn new() -> Self {
        Self {
            board: [0.0; Self::TOTAL_SPACES],
            column_height: [0; Self::COLS],
        }
    }

    pub fn add_token(&mut self, col: usize, player: Player) -> Option<Point2<usize>> {
        let row = self.column_height[col];
        if row >= Self::ROWS {
            return None;
        }

        let new_token_pos = Point2::new(col, row);

        let index = Self::point_to_index(&new_token_pos);
        self.board[index] = player.to_f32();

        self.column_height[col] += 1;
        Some(new_token_pos)
    }

    pub fn remove_token(&mut self, col: usize) {
        let row = self.column_height[col] - 1;

        let token_pos = Point2::new(col, row);

        let index = Self::point_to_index(&token_pos);
        self.board[index] = 0.0;

        self.column_height[col] -= 1;
    }

    pub fn get_val(&self, point: &Point2<usize>) -> f32 {
        self.board[Self::point_to_index(point)]
    }

    pub fn get_token(&self, point: &Point2<usize>) -> Option<Player> {
        let val = self.get_val(point);
        Player::from_val(val)
    }

    fn point_to_index(point: &Point2<usize>) -> usize {
        point.y * Self::COLS + point.x
    }

    pub fn reset(&mut self) {
        self.board.fill(0.0);
        self.column_height.fill(0);
    }

    pub fn as_tensor(&self, player: &Player) -> Tensor<Bknd, 1> {
        if *player == Player::Blue {
            return Self::board_to_tensor(self.board);
        }

        let mut cloned_board = self.board;
        for i in cloned_board.iter_mut() {
            *i *= -1.0;
        }
        Self::board_to_tensor(cloned_board)
    }
    fn board_to_tensor(board: [f32; Self::TOTAL_SPACES]) -> Tensor<Bknd, 1> {
        Tensor::from_floats(board, &DEVICE)
        // Tensor::from_slice(board.as_slice(), (1, Self::TOTAL_SPACES), device).unwrap()
    }
}
