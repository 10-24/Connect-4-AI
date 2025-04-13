use nalgebra::Point2;
use crate::player::Player;

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

    pub fn place_token(&mut self, col: usize, player: Player) -> Option<Point2<usize>> {
        let row = self.column_height[col];

        if row as usize >= Self::ROWS {
            return None;
        }

        let new_token_pos = Point2::new(col, row);

        let index = Self::point_to_index(&new_token_pos);
        self.board[index] = player.to_val();

        self.column_height[col] += 1;
        Some(new_token_pos)
    }
    
    pub fn get_val(&self, point: &Point2<usize>) -> f32 {
        self.board[Self::point_to_index(point)]
    }

    pub fn get_player_token(&self, point: &Point2<usize>) -> Option<Player> {
        let val = self.get_val(point);
        Player::from_val(val)
    }

    fn point_to_index(point: &Point2<usize>) -> usize {
        point.y * Self::COLS + point.x
    }
    fn reset(&mut self){
        self.board.fill(0.0);
        self.column_height.fill(0);
    }
}
