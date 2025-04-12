use candle_core::Device;
use rayon::{array, iter::{FromParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator as _, ParallelIterator}};
use crate::{connect_four::connect_four_enums::GameOutcome, BATCH_SIZE};

use super::{
    episode::{run_episode, EpisodeResult},
    model::ConnectFourNN,
    optimize_model::optimize_model,
};

pub fn train(model_config: ModelConfig, mut training_config: TrainingConfig) {
    
    let episode_iter = (0..BATCH_SIZE).into_par_iter().map(|_|run_episode(&model_config, &training_config));
    let episode_results: Vec<EpisodeResult> = Vec::from_par_iter(episode_iter);
    
    optimize_model(episode_results,&model_config,&mut training_config);
}

pub struct ModelConfig {
    pub model: ConnectFourNN,
    pub device: Device,
    pub num_data_type: candle_core::DType,
}
pub struct TrainingConfig {
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
