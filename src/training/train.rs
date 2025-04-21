use std::path::Path;

use crate::{
    connect_four::connect_four_enums::Outcome,
    training::{
        memory::{batch, convert_episode_to_training_frames::convert_episode_to_training_frames},
        model::ModelConfig,
        optimize_model::optimize_model::optimize_model,
    },
    ModelWithBackend, BACKEND, DEVICE,
};
use burn::{
    backend::{self, Autodiff},
    config::Config,
    nn::loss::{self, MseLoss},
    optim::{self, adaptor::OptimizerAdaptor, Adam, AdamConfig},
    tensor::{backend::AutodiffBackend, Device, Tensor},
};
use rayon::{
    array,
    iter::{
        FromParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator as _,
        ParallelIterator,
    },
};

use super::{run_episode::run_episode, memory::batch::Batch, model::Model};

pub const NUM_BATCHES: u16 = 3000;
pub const BATCH_SIZE: usize = 1;
const LEARNING_RATE: f64 = 3.6;
const EPSILON: f32 = 0.2;
const GAMMA: f32 = 0.97;
pub const WIN_REWARD: f32 = 1.0;
pub const LOSS_REWARD: f32 = -0.9;
pub const TIE_REWARD: f32 = -0.15;
const MODEL_WEIGHTS_PATH: &str = "model.safetensors";

pub fn train() {
    let model = ModelConfig::new().init(&DEVICE);

    println!("Starting Training");

    let training_config = TrainingConfig {
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
        learning_rate: 1e-4,
    };
    let mut optimizer = AdamConfig::new().init();
    for i in 0..NUM_BATCHES {
        let episodes = (0..BATCH_SIZE)
            .into_par_iter()
            .map(|_| run_episode(&model, &training_config));
        let training_frames = episodes
            .map(|episode| convert_episode_to_training_frames(episode, &training_config))
            .reduce(Vec::new, |mut reducer, mut val| {
                reducer.append(&mut val);
                reducer
            });
        let batch = Batch::new(training_frames);

        // let avg_cost = optimize_model(batch,&model_config,&mut training_config);
        // var_map.save(MODEL_WEIGHTS_PATH);
        // println!("Batch: {}/{}, Avg Cost: {}",i,NUM_BATCHES,avg_cost);
    }
    let batch = Batch::from(Path::new("episode_data\\Batch_04-17_18-37"));

    for i in 0..2 {
        // print_varmap_weights(&varmap);

        let batch = &batch;
        let _ = optimize_model(batch, &model, &mut optimizer, &training_config);

        // println!("Batch: {}, Avg Cost: {}",i,avg_cost);
    }
}

pub struct TrainingConfig {
    pub epsilon: f32,
    pub gamma: f32,
    pub learning_rate: f32,
}
