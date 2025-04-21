use burn::tensor::{Int, Tensor};
use csv::{Error, Reader};
use rayon::iter::{FromParallelIterator, ParallelIterator};
use serde::Serialize;
use std::{
    array,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::{
    connect_four::{connect_four_enums::Outcome, game_board::GameBoard, player::Player},
    training::{
        memory::episode_memory::GameFrame, model::target_q_val_builder::TargetQValBuilder, train::{TrainingConfig, BATCH_SIZE, NUM_BATCHES}
    },
    Bknd, DEVICE,
};
use chrono::Local;

use super::episode_memory::{self, EpisodeMemory, TrainingFrame};
use std::fs;

pub struct Batch {
    training_frames: Vec<TrainingFrame>,
}
impl Batch {

    pub fn new(training_frames: Vec<TrainingFrame>) -> Self {
        Self { training_frames }
    }

    pub fn get_game_states(&self) -> Tensor<Bknd, 2> {
        let mut board = GameBoard::new();

        let default_tensor: Tensor<Bknd, 1> = Tensor::empty([GameBoard::TOTAL_SPACES], &DEVICE);
        let mut game_states = vec![default_tensor; self.training_frames.len()];

        let current_episode = self.training_frames[0].episode_id;

        for (training_frame, game_state_val) in
            self.training_frames.iter().zip(game_states.iter_mut())
        {
            if current_episode != training_frame.episode_id {
                board.reset();
            }
            let current_player = training_frame.player;
            *game_state_val = board.as_tensor(&current_player);
            board.add_token(training_frame.col, current_player);
        }

        Tensor::stack(game_states, 0)
    }
    
    pub fn create_target_q_val_builder(&self) -> TargetQValBuilder<Bknd> {
        TargetQValBuilder::new(self.get_selected_cols(), self.get_state_values())
    }

    fn get_selected_cols(&self) -> Tensor<Bknd,1,Int> {
        let selected_cols_vec:Vec<_> = self.training_frames.iter().map(|frame|frame.col).collect();
        Tensor::from_ints(selected_cols_vec.as_slice(), &DEVICE)
    }

    /** The actual value of each action. */
    fn get_state_values(&self) -> Tensor<Bknd,1> {
        let state_values:Vec<_> = self.training_frames.iter().map(|frame|frame.value).collect();
        Tensor::from_floats(state_values.as_slice(), &DEVICE)
    }

    // pub fn save(&self, folder: &Path) {
    //     let timestamp = Local::now().format("%m-%d_%H-%M").to_string();
    //     let batch_folder_name = format!("Batch_{timestamp}");
    //     let batch_folder = folder.join(batch_folder_name);
    //     let _ = fs::create_dir(batch_folder.clone());

    //     for (i, memory) in self.training_frames.iter().enumerate() {
    //         let file_name = format!("Episode_{i}.csv");
    //         let path = batch_folder.join(file_name);
    //         memory.save(&path);
    //     }
    // }

    // pub fn from_folder(batches_folder: &Path) -> Vec<Self> {
    //     let mut batches = Vec::with_capacity(NUM_BATCHES as usize);

    //     let dir = fs::read_dir(batches_folder).unwrap();
    //     for entry in dir.into_iter() {
    //         let entry = entry.unwrap();
    //         let new_batch = Self::from(&entry.path());

    //         batches.push(new_batch);
    //     }

    //     batches
    // }
}
