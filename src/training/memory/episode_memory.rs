use core::f32;
use std::{default, fs::File, io::Read, option::Iter, path::Path};

use burn::tensor::{Int, Tensor};
use derive_new::new;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    connect_four::{connect_four_enums::Outcome, game_board::GameBoard, player::Player},
    training::{run_episode::Identifier, train::{TrainingConfig, BATCH_SIZE}},
    Bknd, DEVICE,
};

use super::batch::Batch;

#[derive(Debug)]
pub struct EpisodeMemory {
    pub frames: Vec<GameFrame>,
    pub id:u16,
}
impl EpisodeMemory {

    pub fn new(id:u16) -> Self {
        Self {
            id,
            frames: Vec::with_capacity(42),
        }
    }

    pub fn record_frame(&mut self, player: Player, selected_col: usize) {
        let new_frame = GameFrame::new(player, selected_col);
        self.frames.push(new_frame);
    }

    pub fn get_actions(&self, player: &Player) -> Tensor<Bknd, 1, Int> {
        let actions: Vec<_> = self
            .frames
            .iter()
            .filter(|turn| turn.player == *player)
            .map(|turn| turn.col as i32)
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
    pub fn get(&self, i: usize) -> &GameFrame {
        &self.frames[i]
    }
    pub fn len(&self) -> usize {
        self.frames.len()
    }
    pub fn from_batch(batch: Batch) -> Vec<Self> {
        let mut episodes = Vec::with_capacity(BATCH_SIZE);

        let mut current_episode = EpisodeMemory::new(batch.training_frames[0].episode_id);
        for training_frame in batch.training_frames {
            if training_frame.episode_id != current_episode.id {
                episodes.push(current_episode);
                current_episode = EpisodeMemory::new(training_frame.episode_id);
            }
            current_episode.frames.push(training_frame.to_game_frame());
        }
        episodes.push(current_episode);
        episodes
    }
}

#[derive(new, Debug, Clone, Copy)]
pub struct GameFrame {
    pub player: Player,
    pub col: usize,

}

#[derive(Debug, Clone, Copy,Default,Serialize,Deserialize)]
pub struct TrainingFrame {
    pub col: usize,
    pub value: f32,
    pub player: Player,
    pub episode_id:u16,
    pub batch_id:u16,
}
impl TrainingFrame {
    pub fn new(batch_id:u16,episode_id:u16,game_frame: GameFrame, value: f32) -> Self {
        let GameFrame {
            player,
            col,
        } = game_frame;
        Self {
            batch_id,
            episode_id,
            col,
            value,
            player,
        }
    }
    fn to_game_frame(self) -> GameFrame {
        let Self {
            player,
            col,
            ..
        } = self;
        GameFrame::new(player, col)
    }
}
