


use crossterm::{
    event::{self, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode}
};

use colored::Colorize;
mod connect_four;
use crate::connect_four::ConnectFour;

fn main() {
    let mut game = ConnectFour::new();
    loop {
        let col = get_column(&game);
        let won = game.play(col);

        if won {
            break;
        }
        game.swap_players();
    }
    print_victory_text(&game)
}
fn get_column(game: &connect_four::ConnectFour) -> i8 {
    let mut selected_col: i8 = 3;

    loop {
        clear_terminal();
        print_player(game.player);
        print_board(game, selected_col);
        

        let key = listen_for_key().unwrap();
        match key {
            KeyPress::Left => selected_col = (selected_col - 1) % 7,
            KeyPress::Right => selected_col = (selected_col + 1) % 7,
            KeyPress::PlaceToken => break,
        }
    }
    selected_col
}
fn print_player(player: connect_four::Player) {
    let mut blue_player: colored::ColoredString = " Player 1 ".blue();
    let mut red_player: colored::ColoredString = " Player 2 ".red();

    if let connect_four::Player::Blue = player {
        blue_player = blue_player.white().on_blue();
    } else {
        red_player = red_player.white().on_red();
    }

    println!("{} | {}", blue_player, red_player);
}
fn print_board(game: &ConnectFour, selected_col: i8) {
    
    for y in (0..game.board[0].len()).rev() {
       
        print!("{} |",  y.to_string().green());
        for x in 0..game.board.len() {
            let current_token = game.board[x][y];

            let mut area = match current_token {
                connect_four::Token::Empty => " X ".dimmed(),
                connect_four::Token::Base(connect_four::Player::Blue) => " 0 ".on_bright_blue(),
                connect_four::Token::Base(connect_four::Player::Red) => " 0 ".on_bright_red(),
            };

            if x == selected_col as usize && y == game.column_height[x] as usize {
                area = match game.player {
                    connect_four::Player::Blue => area.on_bright_black().blue(),
                    connect_four::Player::Red => area.on_bright_black().red(),
                }
            }
            print!(" {}", area)
        }
        print!("|\n\n");
    }
    print!("  ");
    for x in 0..game.board.len() {
        print!("   {}",x.to_string().green());
    }
    println!()
}
fn listen_for_key() -> Result<KeyPress, std::io::Error> {
    enable_raw_mode()?; // Enable raw mode to capture real-time input
    loop {
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                // Only handle KeyDown events (if supported) or avoid repeated events
                if key_event.kind == crossterm::event::KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Left => {
                            disable_raw_mode()?; // Disable raw mode before returning
                            return Ok(KeyPress::Left);
                        }
                        KeyCode::Right => {
                            disable_raw_mode()?; // Disable raw mode before returning
                            return Ok(KeyPress::Right);
                        }
                        KeyCode::Char(' ') => {
                            disable_raw_mode()?; // Disable raw mode before returning
                            return Ok(KeyPress::PlaceToken);
                        }
                        _ => continue, // Ignore other keys
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum KeyPress {
    Right,
    Left,
    PlaceToken,
}
fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn print_victory_text(game: &ConnectFour) {
    print_board(game, 0);

    let text = match game.player {
        connect_four::Player::Blue => "Player Blue Won".on_bright_blue(),
        connect_four::Player::Red => "Player Red Won".on_bright_red(),
    };

    println!("{}", text);
}
