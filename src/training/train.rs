
use std::path::Path;

use rayon::{array, iter::{FromParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator as _, ParallelIterator}};
use crate::{connect_four::connect_four_enums::GameOutcome};
use candle_core::{DType, Device};
use candle_nn::{AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};

use super::{
    episode::run_episode, memory::batch::Batch, model::ConnectFourNN, optimize_model::optimize_model
};

pub const NUM_BATCHES: u16 = 1;
pub const BATCH_SIZE: usize = 10;
const LEARNING_RATE: f64 = 0.0001;
const EPSILON: f32 = 0.88;
const GAMMA: f32 = 0.91;
const MODEL_WEIGHTS_PATH:&str = "model.safetensors";

pub fn train() {
    //Setup
    let device = Device::cuda_if_available(0).unwrap();
    let mut var_map = VarMap::new();
    var_map.load(MODEL_WEIGHTS_PATH).expect("Unable to load weights");

    let vb = VarBuilder::from_varmap(&var_map, DType::F32, &device);
    let optimizer_config = ParamsAdamW {
        lr: LEARNING_RATE,
        ..Default::default()
    };
    
    let optimizer = AdamW::new(var_map.all_vars(), optimizer_config).unwrap();
    
    let model = ConnectFourNN::new(vb, device.clone()).expect("unable to init nn");

    let model_config = ModelConfig { device, model };

    let mut training_config = TrainingConfig {
        optimizer,
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
    };

    //Training
    let episode_iter = (0..BATCH_SIZE).into_par_iter().map(|_|run_episode(&model_config, &training_config));
    let batch = Batch::from_par_iter(episode_iter);
    batch.save(Path::new("episode_data"));
    optimize_model(batch,&model_config,&mut training_config);
    var_map.save(MODEL_WEIGHTS_PATH);
    
}

pub struct ModelConfig {
    pub model: ConnectFourNN,
    pub device: Device,
}
pub struct TrainingConfig {
    // pub rand_gen:Rand
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
