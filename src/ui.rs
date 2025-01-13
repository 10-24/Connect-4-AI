use std::cmp::min;

use egui::{Button, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::connect_four::{ConnectFour, Player, Token};

pub fn create_window() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyEguiApp::new()))),
    );
}
struct MyEguiApp {
    game:ConnectFour,
    victorious_player:Option<Player>,
}
impl MyEguiApp{
    fn new() -> MyEguiApp{
        MyEguiApp { 
            game: ConnectFour::new(),
            victorious_player:None,
        }
    }
    fn reset(&mut self){
        self.game = ConnectFour::new();
        self.victorious_player = None;
    }
}
impl eframe::App for MyEguiApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
       
        egui::CentralPanel::default().show(ctx, |ui| {
            let board_options = BoardOptions {
                mx: 20.0,
                my: 110.0,
                token_radius: 27.0,
                token_gap: 17.0,
                stroke: Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(9, 9, 11),
                },
            };
            
            create_empty_board(ui, &board_options, &self.game);
            populate_board(ui, &board_options, &self.game);
            if let Some(victorious_player) = self.victorious_player {
                let button =  create_reset_button(ui,  &victorious_player);
                if button.clicked() {
                    self.reset();
                }
                return;
            }
            render_moveable_token(ui, &board_options, &self.game);
            if is_clicked(ui) {
                let selected_col = get_selected_column(ui, &board_options, &self.game);
                let won = self.game.play(selected_col);
                if won {
                    self.victorious_player = Some(self.game.current_player);
                    return;
                }
                self.game.swap_players();
            }
        });
    }
}

struct BoardOptions {
    mx: f32,
    my: f32,
    token_radius: f32,
    token_gap: f32,
    stroke: Stroke,
}

fn create_empty_board(ui: &mut Ui, board_options: &BoardOptions, game: &ConnectFour) {
    let board = Rect {
        min: Pos2 {
            x: board_options.mx,
            y: board_options.my,
        },
        max: Pos2 {
            x: board_options.mx
                + board_options.token_gap
                + (game.board.len() as f32)
                    * ((board_options.token_radius * 2.0) + board_options.token_gap),
            y: board_options.my
                + board_options.token_gap
                + (game.board[0].len() as f32)
                    * ((board_options.token_radius * 2.0) + board_options.token_gap),
        },
    };

    ui.painter()
        .rect_stroke(board, egui::Rounding::same(14.0), board_options.stroke);
}

fn populate_board(ui: &mut Ui, board_options: &BoardOptions, game: &ConnectFour) {
    let mx = board_options.mx;
    let my = board_options.my;
    let token_gap = board_options.token_gap;
    let token_radius = board_options.token_radius;
    let token_diameter = token_radius * 2.0;
    let stroke = board_options.stroke;
    let board = game.board;
    let mut pos = Pos2 {
        x: mx + token_gap + token_radius,
        y: my + token_gap + token_radius,
    };

    for y in (0..board[0].len()).rev() {
        for x in 0..board.len() {
            
            let token = board[x][y];
            let token_color = get_color(token);
            ui.painter()
                .circle(pos, token_radius, token_color, stroke);

            pos.x += token_gap + token_diameter;
        }
        pos.x = mx + token_gap + token_radius; //Resetting postion
        pos.y += token_gap + token_diameter;
    }
    fn get_color(token:Token) -> Color32{
        match token {
            Token::Base(Player::Blue) => Color32::BLUE,
            Token::Base(Player::Red) => Color32::RED,
            Token::Empty => Color32::TRANSPARENT,
        }
    }
}
fn render_moveable_token(ui: &mut Ui, board_options: &BoardOptions, game: &ConnectFour) {
    
    let center = Pos2 {
        x: get_token_x_pos(ui, board_options, game),
        y: 60.0,
    };

    let fill = match game.current_player {
        Player::Blue => Color32::BLUE,
        Player::Red => Color32::RED,
    };
    ui.add(CircleShapeWidget {
        center,
        fill,
        radius:board_options.token_radius,
    });

    fn get_token_x_pos(ui: &mut Ui, board_options: &BoardOptions, game: &ConnectFour) -> f32 {
        let token_radius = board_options.token_radius;
        let token_gap = board_options.token_gap;
        let token_diameter = token_radius * 2.0;
        let leftward_margin = board_options.mx + token_gap / 2.0;

        let mouse_col = get_selected_column(ui, board_options, game) as f32;


        leftward_margin + token_radius + token_gap / 2.0 + mouse_col * (token_gap + token_diameter)
    }
}
fn get_selected_column(ui: &mut Ui, board_options: &BoardOptions, game: &ConnectFour) -> i8 {
    let leftward_margin = board_options.mx + board_options.token_gap / 2.0;
    let token_diameter = board_options.token_radius * 2.0;
    let col_width = token_diameter + board_options.token_gap;
    let mouse_x = ui
        .input(|input| input.pointer.hover_pos())
        .map(|pos| pos.x)
        .unwrap_or(0.0);

    let col = ((mouse_x - leftward_margin) / col_width).floor() as i8;
    
    
    min(col,game.board.len() as i8)
}
fn create_reset_button(ui: &mut Ui, victorious_player: &Player) -> Response {
    let text = format!("Player {} won! Reset?", victorious_player);
   
    let button = Button::new(text);
    ui.add(button).to_owned()
}
struct CircleShapeWidget {
    center: Pos2,
    fill: Color32,
    radius: f32,
}
impl egui::Widget for CircleShapeWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.painter()
            .circle_filled(self.center, self.radius, self.fill);
        let diameter = 2.0 * self.radius;
        let size = Vec2 {
            x: diameter,
            y: diameter,
        };
        let sense = Sense {
            click: false,
            drag: false,
            focusable: false,
        };
        let (_, response) = ui.allocate_exact_size(size, sense);
        response
    }
}

fn is_clicked(ui: &mut Ui,) -> bool{
    ui.input(|state|{
        state.pointer.primary_clicked() 
    })
}