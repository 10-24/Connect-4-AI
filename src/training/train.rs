use crate::{
    training::{memory::episode_memory::EpisodeMemory, model::model_config::ModelConfig, optimize_model::optimize_model::optimize_model},
    ModelWithBackend, DEVICE,
};
use burn::{
    module::Module,
    optim::AdamConfig,
    record::{DefaultFileRecorder, FullPrecisionSettings, NamedMpkFileRecorder, Recorder},
};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{path::Path, sync::Arc};

use super::{
    memory::batch::{Batch, BatchFile},
    run_episode::run_episode,
};

pub const NUM_BATCHES: u16 = 1000;
pub const EPISODES_PER_BATCH: usize = 6;

const EPSILON: f32 = 0.65;
const GAMMA: f32 = 0.97;
const MODEL_WEIGHTS_PATH: &str = "model.safetensors";
const BATCH_FOLDER: &str = "batches";
pub fn train() {
    let model_parameters_recorder: NamedMpkFileRecorder<FullPrecisionSettings> =
        DefaultFileRecorder::new();
    let mut model = ModelConfig::new().init(&DEVICE);

    let training_config = TrainingConfig {
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
        learning_rate: 0.0003,
    };
    let mut optimizer = AdamConfig::new().init();
    let mut batch_file = BatchFile::new();

    for batch_id in 1..=NUM_BATCHES {
       
        let episodes = (0..EPISODES_PER_BATCH).into_par_iter().map(|episode_id| {
            let model = ModelConfig::new().init(&DEVICE);
            run_episode(episode_id as u16, &model, &training_config)
        }).collect();

        let batch = Batch::from_episodes(batch_id, episodes, &training_config);

        let (new_model, loss) =
            optimize_model(batch.clone(), model, &mut optimizer, &training_config);

        model = new_model;
        println!("batch {batch_id}/{NUM_BATCHES}, loss {loss}");
        batch_file.add(batch);
    }

    batch_file.save(BATCH_FOLDER).expect("Couldn't save batch");
    model
        .save_file(MODEL_WEIGHTS_PATH, &model_parameters_recorder)
        .expect("Unable to save model");
}

#[derive(Clone, Copy)]
pub struct TrainingConfig {
    pub epsilon: f32,
    pub gamma: f32,
    pub learning_rate: f32,
}
