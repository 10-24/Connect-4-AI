use crate::{
    training::{
        memory::convert_episode_to_training_frames::convert_episode_to_training_frames,
        model::model_config::ModelConfig, optimize_model::optimize_model::optimize_model,
    },
    ModelWithBackend, DEVICE,
};
use burn::{
    module::Module,
    optim::AdamConfig,
    record::{DefaultFileRecorder, FullPrecisionSettings, NamedMpkFileRecorder, Recorder},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path::Path, sync::Arc};

use super::{memory::batch::{Batch, BatchFile}, run_episode::run_episode};

pub const NUM_BATCHES: u16 = 1000;
pub const BATCH_SIZE: usize = 6;

const EPSILON: f32 = 0.65;
const GAMMA: f32 = 0.97;
const MODEL_WEIGHTS_PATH: &str = "model.safetensors";

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
    let mut batch_file = BatchFile::new("episode_data");

    for i in 1..=NUM_BATCHES {
        let episodes = (0..BATCH_SIZE).into_par_iter().map(|_| {
            let model = ModelConfig::new().init(&DEVICE);
            run_episode(&model, &training_config)
        });

        let training_frames = episodes
            .map(|episode| convert_episode_to_training_frames(episode, &training_config))
            .reduce(Vec::new, |mut reducer, mut val| {
                reducer.append(&mut val);
                reducer
            });
        let batch = Batch::new(training_frames);

        let (new_model, loss) =
            optimize_model(batch.clone(), model, &mut optimizer, &training_config);

            model = new_model;
        println!("batch {i}/{NUM_BATCHES}, loss {loss}");
        batch_file.add(batch);
    }
    
    batch_file.save();
    model.save_file(MODEL_WEIGHTS_PATH, &model_parameters_recorder).expect("Unable to save model");
}

#[derive(Clone, Copy)]
pub struct TrainingConfig {
    pub epsilon: f32,
    pub gamma: f32,
    pub learning_rate: f32,
}
