use candle_core::Device;
use candle_nn::{AdamW, Optimizer};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::connect_four::connect_four_enums::GameOutcome;

use super::{
    episode::{run_episode, EpisodeSetup},
    model::ConnectFourNN,
    optimize_model::{optimize_model, TrainingFrame},
};

pub fn train(model_config: ModelConfig, training_config: TrainingConfig) {
    let ModelConfig { model, device } = model_config;
    let TrainingConfig {
        batch_size,
        num_batches,
        epsilon,
        gamma,
        mut optimizer,
    } = training_config;
    
    let episode_setup = EpisodeSetup {
        epsilon,
        model,
        device: device.clone(),
    };

    for _ in 0..num_batches {
        let training_frames: Vec<TrainingFrame> = (0..batch_size)
            .into_par_iter()
            .flat_map(|_| {
                let result = run_episode(&episode_setup);
                optimize_model(&result, gamma.clone(), &device)
            })
            .collect();

        training_frames.iter().for_each(|frame| {
            optimizer.backward_step(&frame.cost);
        });
    }
}

pub struct ModelConfig {
    pub model: ConnectFourNN,
    pub device: Device,
    pub num_data_type: candle_core::DType,
}
pub struct TrainingConfig {
    pub batch_size: u16,
    pub num_batches: u16,
    pub epsilon: f32, // Exploration rate
    pub gamma: f32,   // Discount factor
    pub optimizer: candle_nn::AdamW,
}

impl GameOutcome {
    pub fn reward(&self) -> f32 {
        match &self {
            GameOutcome::Win => 1.0,
            GameOutcome::Tie => -0.05,
            GameOutcome::Loss => -1.0,
        }
    }
}
