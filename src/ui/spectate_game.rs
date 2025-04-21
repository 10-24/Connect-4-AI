use derive_new::new;
use egui::{Color32, Key, Label, Pos2, Response, RichText, Sense, Stroke, Ui};

use crate::{connect_four::game_board::GameBoard, player::Player, training::memory::episode_memory::{EpisodeMemory, GameFrame}};

use super::render_board::{render_board, BoardRenderOptions};

pub fn spectate_game(episode:EpisodeMemory){
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Connect 4",
        native_options,
        Box::new(|_cc| Ok(Box::new(EguiSpectatorWindow::new(episode)))),
    );
}

struct EguiSpectatorWindow {
    turn:usize,
    board:GameBoard,
    episode: EpisodeMemory,
}
impl EguiSpectatorWindow {
    fn new(episode: EpisodeMemory) -> Self {
        Self {
            episode,
            board: GameBoard::new(),
            turn:0,
        }
    }

    fn _print_turn(&self){
        // let turn_index = (self.episode.len() - 1).min(self.turn + 1);
        // let player = self.episode.get(turn_index).player;
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

            if is_clicked(ui, Key::ArrowRight) && self.turn < self.episode.len()  {
                let turn = self.episode.get(self.turn);
                self.board.add_token(turn.col, turn.player);
                self.turn += 1;
            } 
            if is_clicked(ui, Key::ArrowLeft) && self.turn > 0  {
                self.turn -= 1;
                let turn = self.episode.get(self.turn);
                self.board.remove_token(turn.col);
            } 
            
            render_board(ui,&self.board,&board_options);
        });
    }

    
}



fn is_clicked(ui: &mut Ui,key:Key) -> bool {
    ui.input(|state| state.key_released(key))
}

