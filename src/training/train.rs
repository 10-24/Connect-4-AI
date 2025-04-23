use std::{path::Path, sync::Arc};
use burn::{module::Module, optim::AdamConfig, record::{DefaultFileRecorder, FullPrecisionSettings, NamedMpkFileRecorder, Recorder}};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::{training::{memory::convert_episode_to_training_frames::convert_episode_to_training_frames, model::model_config::ModelConfig, optimize_model::optimize_model::optimize_model}, ModelWithBackend, DEVICE};

use super::{memory::batch::Batch, run_episode::run_episode};

pub const NUM_BATCHES: u16 = 1000;
pub const BATCH_SIZE: usize = 6;
pub const SAVE_INTERVAL:u16 = 20;

const EPSILON: f32 = 0.65;
const GAMMA: f32 = 0.97;
const MODEL_WEIGHTS_PATH: &str = "model.safetensors";

pub fn train() {
    
    let model_parameters_recorder: NamedMpkFileRecorder<FullPrecisionSettings> = DefaultFileRecorder::new();
    let mut model = ModelConfig::new().init(&DEVICE);
    
    let training_config = TrainingConfig {
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
        learning_rate: 0.0003,
    };
    let mut optimizer = AdamConfig::new().init();

    for i in 1..=NUM_BATCHES {
        
        let episodes = (0..BATCH_SIZE).into_par_iter()
            .map(|_| {
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
        

        batch.save(Path::new("episode_data"),i.to_string().as_str());
        
        let (new_model,loss) = optimize_model(batch.clone(),model,&mut optimizer,&training_config,);
        println!("batch {i}/{NUM_BATCHES}, loss {loss}");
        model = new_model;
        if i % SAVE_INTERVAL == 0 {
            model.clone().save_file(MODEL_WEIGHTS_PATH, &model_parameters_recorder).expect("Cannot save model parameters");
        }
    }
}

#[derive(Clone, Copy)]
pub struct TrainingConfig {
    pub epsilon: f32,
    pub gamma: f32,
    pub learning_rate: f32,
}
