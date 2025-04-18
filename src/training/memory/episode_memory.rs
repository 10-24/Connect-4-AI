use std::{fs::File, io::Read, option::Iter, path::Path};

use candle_core::{Device, Tensor};
use csv::Reader;
use derive_new::new;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::connect_four::{
    connect_four::ConnectFour, connect_four_enums::GameOutcome, game_board::GameBoard,
    player::Player,
};

#[derive(Serialize)]
pub struct EpisodeMemory {
    turns: Vec<GameTurn>,
}
impl EpisodeMemory {
    pub fn new() -> Self {
        Self {
            turns: Vec::with_capacity(12),
        }
    }

    pub fn record_turn(&mut self, player: Player, selected_col: u8) {
        let new_turn = GameTurn::new(player, selected_col);
        self.turns.push(new_turn)
    }

    pub fn get_game_states(&self, player_perspective: &Player, device: &Device) -> Tensor {
        let mut game_states = Vec::with_capacity(self.turns.len() / 2 + 1);
        
        let mut board_template = GameBoard::new();
        for turn in self.turns.iter() {
            if turn.player != *player_perspective {
                board_template.add_token(turn.col, turn.player);
                continue;
            }
            let board_tensor: Tensor = board_template.get_board_tensor(player_perspective, device);
            game_states.push(board_tensor);
            board_template.add_token(turn.col, turn.player);
        }

        Tensor::cat(game_states.as_slice(), 0).unwrap()
    }
    pub fn get_actions(&self, player: &Player, device: &Device) -> Tensor {
        let iter = self
            .turns
            .iter()
            .filter(|turn| turn.player == *player)
            .map(|turn| turn.col as u32);
        Tensor::from_iter(iter, device)
            .unwrap()
            .unsqueeze(1)
            .unwrap()
    }
    pub fn number_of_turns(&self, player: &Player) -> usize {
        self.turns
            .iter()
            .filter(|turn| turn.player == *player)
            .count()
    }
    pub fn outcome(&self, player: &Player) -> GameOutcome {
        if self.turns.len() == GameBoard::TOTAL_SPACES {
            return GameOutcome::Tie;
        }
        let last_player = self.turns.last().unwrap().player;
        if last_player == *player {
            return GameOutcome::Win;
        }
        GameOutcome::Loss
    }
    pub fn save(&self, path: &Path) {
        let file = File::create(path).unwrap();

        let mut writer = csv::Writer::from_writer(file);

        for turn in self.turns.iter() {
            writer.serialize(turn);
        }

        writer.flush();
    }

    pub fn from_path(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        let turns: Vec<GameTurn> = reader.deserialize().map(|result| result.unwrap()).collect();
        Self { turns }
    }
    pub fn get(&self, index: usize) -> &GameTurn {
        &self.turns[index]
    }
    pub fn len(&self) -> usize {
        self.turns.len()
    }
}

#[derive(new, Serialize, Deserialize)]
pub struct GameTurn {
    pub player: Player,
    pub col: u8,
}
