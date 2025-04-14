use derive_new::new;
use egui::{Color32, Key, Label, Pos2, Response, RichText, Sense, Stroke, Ui};

use crate::{connect_four::game_board::GameBoard, player::Player};

use super::render_board::{render_board, BoardRenderOptions};

pub fn spectate_game(game_record:Vec<GameTurn>){
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Connect 4",
        native_options,
        Box::new(|_cc| Ok(Box::new(EguiSpectatorWindow::new(game_record)))),
    );
}

struct EguiSpectatorWindow {
    turn:usize,
    board:GameBoard,
    game_record: Vec<GameTurn>,
}
impl EguiSpectatorWindow {
    fn new(game_record: Vec<GameTurn>) -> Self {
        Self {
            game_record,
            board: GameBoard::new(),
            turn:0,
        }
    }

    fn create_header(&self,ui:&mut Ui){
        let turn_index = (self.game_record.len() - 1).min(self.turn + 1);
        let player = self.game_record[turn_index].player;
        let text = format!("Player {}'s Turn. {}/{} ",player.to_string(),turn_index,self.game_record.len() -1 );

        ui.add(Label::new(RichText::new(text).size(25.0)));
    }
}
impl eframe::App for EguiSpectatorWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx, |ui| {
            let board_options = BoardRenderOptions {
                mx: 20.0,
                my: 110.0,
                token_radius: 27.0,
                token_gap: 17.0,
                stroke: Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(9, 9, 11),
                },
            };

            if is_clicked(ui, Key::ArrowRight) && self.turn < (self.game_record.len() - 1)  {
                self.turn += 1;
                let turn = &self.game_record[self.turn];
                self.board.add_token(turn.action, turn.player);
            } 
            if is_clicked(ui, Key::ArrowLeft) && self.turn > 0  {
                let turn = &self.game_record[self.turn];
                self.board._remove_token(turn.action);
                self.turn -= 1;
            } 
            self.create_header(ui);
            render_board(ui,&self.board,&board_options);
        });
    }

    
}



fn is_clicked(ui: &mut Ui,key:Key) -> bool {
    ui.input(|state| state.key_released(key))
}

#[derive(new)]
pub struct GameTurn {
    player:Player,
    action:usize,
}