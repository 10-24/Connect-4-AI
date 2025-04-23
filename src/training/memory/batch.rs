use std::{
    fs::{self, File}, io::Write, path::Path
};

use burn::tensor::{Int, Tensor};
use chrono::Local;
use derive_new::new;
use serde::Serialize;

use crate::{
    connect_four::game_board::GameBoard,
    training::{
        model::target_q_val_builder::TargetQValBuilder,
        train::{BATCH_SIZE, NUM_BATCHES},
    },
    Bknd, DEVICE,
};

use super::episode_memory::TrainingFrame;

#[derive(Debug, Clone, new, Default)]
pub struct Batch {
    pub training_frames: Vec<TrainingFrame>,
}
impl Batch {
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

    fn get_selected_cols(&self) -> Tensor<Bknd, 1, Int> {
        let selected_cols_vec: Vec<_> =
            self.training_frames.iter().map(|frame| frame.col).collect();
        Tensor::from_ints(selected_cols_vec.as_slice(), &DEVICE)
    }

    /** The actual value of each action. */
    fn get_state_values(&self) -> Tensor<Bknd, 1> {
        let state_values: Vec<_> = self
            .training_frames
            .iter()
            .map(|frame| frame.value)
            .collect();
        Tensor::from_floats(state_values.as_slice(), &DEVICE)
    }

    pub fn to_csv(&self) -> Vec<u8> {
        let mut csv_writer = csv::Writer::from_writer(Vec::with_capacity(self.training_frames.len()));
        self.training_frames.iter().for_each(|frame|
            csv_writer.serialize(frame).expect("Cannot serialize turn")
        );
        csv_writer.into_inner().unwrap()
    }

    pub fn from_file(path: &Path) -> Self {
        let mut csv_reader = csv::Reader::from_path(path).unwrap();
        let training_frames: Vec<TrainingFrame> = csv_reader
            .deserialize()
            .map(|frame| frame.expect("Unable to deserialized training frame"))
            .collect();

        Self { training_frames }
    }
}

#[derive(Debug)]
pub struct BatchFile {
    batches: Vec<Batch>,
    file_path_string: String,
}
impl BatchFile {
    pub fn new(folder: &str) -> Self {
        let file_name = Self::create_file_name();
        let file_path_string = format!("{folder}/{file_name}");

        let batches = Vec::with_capacity(NUM_BATCHES as usize);
        
        Self {
            batches,
            file_path_string,
        }
    }
    pub fn add(&mut self, batch: Batch) {
        self.batches.push(batch);
    }
    pub fn save(self) {
        
        let file = File::create_new(self.file_path_string).expect("Couldn't create a new file");
        let csv:Vec<u8> = self.batches.iter().map(|batch| 
            batch.to_csv()
        ).reduce(|&mut a, &mut b|{}).collect();
        file.write(buf)
    }
    fn create_file_name() -> String {
        let timestamp = Local::now().format("%m-%d_%H-%M").to_string();
        format!("Batch_{timestamp}")
    }
}
