use std::num;

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
                let checks = [
                    self.check_for_win(CheckStructure {
                        x,
                        y,
                        x_inc: 1,
                        y_inc: 0,
                    }),
                    self.check_for_win(CheckStructure {
                        x,
                        y,
                        x_inc: 0,
                        y_inc: 1,
                    }),
                    self.check_for_win(CheckStructure {
                        x,
                        y,
                        x_inc: 1,
                        y_inc: 1,
                    }),
                    self.check_for_win(CheckStructure {
                        x,
                        y,
                        x_inc: -1,
                        y_inc: -1,
                    }),
                ];
                if let Some(true) = checks.iter().find(|&&x| x) {
                    return true;
                }
            }
        }
        false
    }
    fn check_for_win(&self, check_structure: CheckStructure) -> bool {
        let mut x = get_starting(check_structure.x.pos)
        let mut y = get_starting(check_structure.y, check_structure.y_inc);

        let mut series_len: u8 = 0;

        for iteration in 0..7 {
            if series_len == 4 {
                return true;
            }
            if x == self.board.len() as u8 || y == self.board[0].len() as u8 {
                return false;
            }

            if self.board[x as usize][y as usize] == self.player {
                series_len += 1;
            } else {
                series_len = 0;
            }
            x += check_structure.x_inc;
            y += check_structure.y_inc;
        }

        fn get_starting(pos:u8) -> u8{
            let start = pos - 4;
            if start >= 0 {
                return start;
            }
            return 0;
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
struct CheckStructure {
    x:CheckDemension, //Demension
    y:CheckDemension
}
struct CheckDemension {
    pos:u8,
    increment:i8,
}
