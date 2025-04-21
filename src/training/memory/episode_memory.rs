use core::f32;
use std::{default, fs::File, io::Read, option::Iter, path::Path};

use burn::tensor::{Int, Tensor};
use derive_new::new;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    connect_four::{connect_four_enums::Outcome, game_board::GameBoard, player::Player},
    training::train::TrainingConfig,
    Bknd, DEVICE,
};

pub struct EpisodeMemory {
    pub frames: Vec<GameFrame>,
    pub id: u16,
}
impl EpisodeMemory {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            frames: Vec::with_capacity(42),
        }
    }

    pub fn record_turn(&mut self, player: Player, selected_col: usize) {
        let new_turn = GameFrame::new(player, selected_col);
        self.frames.push(new_turn)
    }

    pub fn get_actions(&self, player: &Player) -> Tensor<Bknd, 1, Int> {
        let actions: Vec<_> = self
            .frames
            .iter()
            .filter(|turn| turn.player == *player)
            .map(|turn| turn.col as i16)
            .collect();
        Tensor::from(actions.as_slice())
    }
    pub fn outcome(&self) -> Outcome {
        if self.frames.len() == GameBoard::TOTAL_SPACES {
            return Outcome::Tie;
        }
        let last_player = self.frames.last().unwrap().player;
        Outcome::Win(last_player)
    }
    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

#[derive(new)]
pub struct GameFrame {
    pub player: Player,
    pub col: usize,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TrainingFrame {
    pub col: usize,
    pub value: f32,
    pub player: Player,
    pub episode_id: u16,
}
impl TrainingFrame {
    pub fn new(episode_id: u16, game_frame: GameFrame, value: f32) -> Self {
        let GameFrame { player, col } = game_frame;
        Self {
            episode_id,
            col,
            value,
            player,
        }
    }
}
