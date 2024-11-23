// use core::unicode::conversions;


use std::io;

use colored::{ColoredString, Colorize};
mod connect_four;
use crate::connect_four::ConnectFour;
// use rustyline::Editor;
fn main(){
    let mut game = ConnectFour::new();
    
    loop {
        clear_terminal();
        let col = get_column(&game);
        let won = game.play(col);
        if won {
            break;
        }
    }
    println!("Player {} | Won! ", game.player.to_string().on_cyan())
}
fn get_column(game:&connect_four::ConnectFour) -> i8 {
    let mut selected_col:i8 = 0;

    loop {
        clear_terminal();
        print_player(game.player);
        print_board(game,selected_col);

        let val = get_selected_col(selected_col, game.board.len().try_into().unwrap());
        
        if val == 0 {
            break;
        }
        selected_col += val;
    }
    selected_col
}
fn print_player(player:i8){
    let mut player1: colored::ColoredString = " Player 1 ".blue();
    let mut player2: colored::ColoredString = " Player 2 ".red();

    if player == 1 {
        player1 = player1.white().on_blue();
    } else {
        player2 = player2.white().on_red();
    }
    
    println!("{} | {}",player1,player2);
}
fn print_board(game:&ConnectFour,selected_col:i8){
    for row in 0..game.board[0].len(){ 
        for col in 0..game.board.len(){

            let val = game.board[col][row];
            let mut token:ColoredString;
            match val {
                0 => token = " X ".dimmed(),
                1 => token = " 1 ".on_blue(),
                2 => token = " 2 ".on_red(),
                _ => token = " E ".bright_magenta(),
            }

            if col == selected_col as usize && row == game.find_token_row(col as i8) as usize {
                token = token.on_bright_black()
            }
            print!(" {}",token)
        }
        print!("\n\n");
    }
}
fn get_selected_col(selected_col:i8,cols:i8)-> i8{
    
        println!("Enter Input");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let key = input.trim();
        println!("received :{}",key);
        if key == ">" && selected_col < cols - 1 {
            return 1;
        }
        if key == "<" && selected_col > 0 {
            return -1;
        }

        0
}


fn clear_terminal(){
    print!("{}[2J", 27 as char);
}