
use std::ops::Neg;

use nalgebra::Point2;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator};

use super::{connect_four::ConnectFour, connect_four_enums::Delta, game_board::GameBoard};



impl ConnectFour {
    const CHECK_DIRECTIONS: [Point2<Delta>; 4] = [
    Point2::new(Delta::Zero, Delta::Increment),
    Point2::new(Delta::Increment, Delta::Zero),
    Point2::new(Delta::Increment, Delta::Increment),
    Point2::new(Delta::Increment, Delta::Decrement),
];
    pub fn player_won(&self, new_token: Point2<usize>) -> bool {
        Self::CHECK_DIRECTIONS
            .par_iter()
            .any(|direction|{ self.check_direction_for_win(&new_token, direction)})
    }

    fn check_direction_for_win(
        &self,
        new_token_pos: &Point2<usize>,
        check_direction: &Point2<Delta>,
    ) -> bool {
        let mut series_len: u8 = 1; // 1 to account for the new token
        series_len += self.march_in_direction(new_token_pos, check_direction);

        let negitive_check_direction = Point2::new(check_direction.x.neg(),check_direction.y.neg());
        series_len += self.march_in_direction(new_token_pos, &negitive_check_direction);

        series_len >= self.victory_threshold
    }

    fn march_in_direction(&self, new_token_pos: &Point2<usize>, direction: &Point2<Delta>) -> u8 {
        let offset = Point2::new(direction.x.to_i8(),direction.y.to_i8());

        let mut current_pos = Point2::new(new_token_pos.x as i8, new_token_pos.y as i8);
        let mut series_len = 0;

        loop {
            current_pos.x += offset.x;
            current_pos.y += offset.y;

            // Checking for underflow
            if current_pos.x < 0 || current_pos.y < 0 {
                return series_len;
            }

            // Checking for overflow
            if current_pos.x as usize >= GameBoard::COLS || current_pos.y as usize >= GameBoard::ROWS {
                return series_len;
            }

            let current_pos_usize = Point2::new(current_pos.x as usize, current_pos.y as usize);
            let current_token = self.board.get_token(&current_pos_usize);
            if current_token != Some(self.current_player) {
                return series_len;
            }
            series_len += 1;
        }
    }
}
