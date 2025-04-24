use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator};

use crate::{connect_four::player::Player, training::train::{TrainingConfig, BATCH_SIZE}};

use super::{batch::Batch, episode_memory::{EpisodeMemory, TrainingFrame}};

impl Batch {
   
    pub fn from_episodes(
        batch_id:u16,
        episodes: [EpisodeMemory;BATCH_SIZE],
        training_config: &TrainingConfig,
    )-> Self{
        let training_frames = episodes.par_iter()
        Self {
            training_frames,
        }
    }
    fn episode_to_training_frames(
        batch_id:u16,
        episode: EpisodeMemory,
        training_config: &TrainingConfig,
    )-> Vec<TrainingFrame> {
        let outcome = episode.outcome();
        let mut blue_current_state_value = outcome.reward(&Player::Blue);
        let mut red_current_state_value = outcome.reward(&Player::Red);
    
        let gamma = training_config.gamma;
        let mut training_frames = vec![TrainingFrame::default(); episode.len()];
    
        for (game_frame, training_frame_val) in episode
            .frames
            .into_iter()
            .zip(training_frames.iter_mut())
            .rev()
        {
            let player = game_frame.player;
    
            let value = if player == Player::Blue {
                let v = blue_current_state_value;
                blue_current_state_value *= gamma;
                v
            } else {
                let v = red_current_state_value;
                red_current_state_value *= gamma;
                v
            };
            *training_frame_val = TrainingFrame::new(batch_id,episode.id,game_frame, value);
        }
        training_frames
    }
    
}