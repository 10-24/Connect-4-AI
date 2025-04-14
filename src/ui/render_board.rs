use egui::{Color32, Pos2, Rect, Stroke, Ui};
use nalgebra::Point2;

use crate::{connect_four::{ game_board::GameBoard}, player::Player};

pub fn render_board(ui: &mut Ui, board:&GameBoard, board_options: &BoardRenderOptions){
    create_empty_board(ui, &board_options);
    populate_board(ui, &board,&board_options,);
}

fn create_empty_board(ui: &mut Ui, board_options: &BoardRenderOptions) {
    let board = Rect {
        min: Pos2 {
            x: board_options.mx,
            y: board_options.my,
        },
        max: Pos2 {
            x: board_options.mx
                + board_options.token_gap
                + (GameBoard::COLS as f32)
                    * ((board_options.token_radius * 2.0) + board_options.token_gap),
            y: board_options.my
                + board_options.token_gap
                + (GameBoard::ROWS as f32)
                    * ((board_options.token_radius * 2.0) + board_options.token_gap),
        },
    };

    ui.painter()
        .rect_stroke(board, egui::Rounding::same(14.0), board_options.stroke);
}

fn populate_board(ui: &mut Ui,board:&GameBoard, board_options: &BoardRenderOptions, ) {

    let BoardRenderOptions { mx,my,token_gap,token_radius,stroke}= board_options;
    let token_diameter = token_radius * 2.0;
   
    
    let mut pos = Pos2 {
        x: mx + token_gap + token_radius,
        y: my + token_gap + token_radius,
    };

    for y in (0..GameBoard::ROWS).rev() {
        for x in 0..GameBoard::COLS {
            let token = board.get_token(&Point2::new(x,y));
            let token_color = get_color(token);
            ui.painter().circle(pos, *token_radius, token_color, *stroke);

            pos.x += token_gap + token_diameter;
        }
        pos.x = mx + token_gap + token_radius; //Resetting postion
        pos.y += token_gap + token_diameter;
    }
    fn get_color(token: Option<Player>) -> Color32 {
        match token {
            Some(Player::Blue) => Color32::BLUE,
            Some(Player::Red) => Color32::RED,
            None => Color32::TRANSPARENT,
        }
    }
}

pub struct BoardRenderOptions {
    pub mx: f32,
    pub my: f32,
    pub token_radius: f32,
    pub token_gap: f32,
    pub stroke: Stroke,
}