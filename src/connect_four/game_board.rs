use derive_new::new;
use nalgebra::Point2;

use crate::player::Player;

pub struct GameBoard {
    pub board: [f32; Self::TOTAL_SPACES],

    pub column_height: [usize; Self::COLS],
}
impl GameBoard {
    pub const COLS: usize = 7;
    pub const ROWS: usize = 6;
    pub const TOTAL_SPACES: usize = Self::COLS * Self::ROWS;

    fn new() -> Self {
        Self {
            board: [0; Self::TOTAL_SPACES],
            column_height: [0; Self::COLS],
        }
    }
    fn place_token(&mut self, col: usize, player: Player) -> bool {
        let row = self.column_height[col];

        if row as usize >= Self::ROWS {
            return false;
        }

        let new_token_pos = Point2::new(col, row);

        let index = Self::point_to_index(&new_token_pos);
        self.board[index] = player.to_val();

        self.column_height[col] += 1;
        true
    }

    pub fn get_val(&self, point: &Point2<usize>) -> i8 {
        self.board[Self::point_to_index(point)]
    }
    pub fn get_player_token(&self, point: &Point2<usize>) -> Option<Player> {
        let val = self.get_val(point);
        Player::from_val(val)
    }
    fn point_to_index(point: &Point2<usize>) -> usize {
        point.y * Self::COLS + point.x
    }
}
