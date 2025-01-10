use std::cmp::{max,min};

pub struct ConnectFour {
    pub board: [[i8; 6]; 7],
    pub player: i8,
}
impl ConnectFour {
    pub fn play(&mut self, column: i8) -> bool {
        self.swap_players();

        let row = self.find_token_row(column);
        self.board[column as usize][row as usize] = self.player;

        self.player_won()
    }
    fn swap_players(&mut self) {
        if self.player == 1 {
            self.player = 2;
        } else {
            self.player = 1;
        }
    }
    pub fn find_token_row(&self, column: i8) -> i8 {
        for row in (0..6).rev() {
            if self.board[column as usize][row] == 0 {
                return row as i8;
            }
        }
        -1
    }
    fn player_won(&self) -> bool {
        for x in 0..self.board.len() - 4 {
            for y in (self.board.len() - 1)..0 {
                let position = Point::new(x as i8, y as i8);
                let checks = [
                    // self.check_for_win(&position,&Delta::Decrement), 
                    self.check_for_win(&position,&Point { x:Delta::Increment,y:Delta::Increment} ),
                    // self.check_for_win(&position,Point::new(1, 0)),
                    // self.check_for_win(&position,Point::new(1, 0)),
                ];

                if let Some(true) = checks.iter().find(|&&x| x) {
                    return true;
                }
            }
        }
        false
    }
    fn check_for_win(&self, position:&Point<i8>, delta_y:&Point<Delta>) -> bool {

        let mut x = get_starting_x(&position.x);
        let mut y = get_starting_y(&position.y, &delta_y);
        let y_inc:i8 = match delta_y {
            Delta::Increment => 1,
            Delta::Zero => 0,
            Delta::Decrement => -1
        };
        let mut series_len: u8 = 0;

        for iteration in 0..7 {
            if series_len == 4 {
                return true;
            }
            if x == self.board.len() as i8 || y == self.board[0].len() as i8 {
                return false;
            }

            if self.board[x.into()][y.into()] == self.player {
                series_len += 1;
            } else {
                series_len = 0; // Clear
            }

            x += 1;
            y += match delta_y {
                Delta::Increment => 1,
                Delta::Zero => 0,
                Delta::Decrement => -1
            }
        }

        return false;

        fn get_starting_x(x_pos:&i8) -> i8 {
            max(x_pos - 4,0)
        }
        fn get_starting_y(y_pos:&i8,delta_y:&Delta)-> i8{
            match delta_y {
                Delta::Increment => max(y_pos - 4,0),
                Delta::Zero => y_pos.clone(),
                Delta::Decrement => min(y_pos + 4 , 6)
            }
        }
        fn get_number_of_col_checks(modified_col: u8) -> u8 {
            //! I think starting_pos should instead be modified col
            let checks = -1 * (modified_col as i8 - 4).abs() + 7;
            return checks as u8;
        }
        fn get_number_of_row_checks(modified_col: u8) -> u8 {
            let checks = -1f32 * ((modified_col as f32) - 2.5).abs() + 6.5;
            checks.floor() as u8
        }
    }
}

impl ConnectFour {
    pub fn new() -> ConnectFour {
        ConnectFour {
            board: [[0; 6]; 7],
            player: 0,
        }
    }
}
struct Point<T> {
    x:T,
    y:T,
}
impl<T> Point<T> {
    pub fn new(x:T,y:T) -> Point<T>{
        Point {x,y,}
    }
}

enum Delta{
    Increment,
    Zero,
    Decrement,
}