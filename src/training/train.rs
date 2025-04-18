
use std::{env::var, path::Path};

use rayon::{array, iter::{FromParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator as _, ParallelIterator}};
use crate::{connect_four::connect_four_enums::GameOutcome, logger::Logger, training::memory::batch};
use candle_core::{DType, Device};
use candle_nn::{AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap, SGD};

use super::{
    episode::run_episode, memory::batch::Batch, model::ConnectFourNN, optimize_model::optimize_model
};

pub const NUM_BATCHES: u16 = 3000;
pub const BATCH_SIZE: usize = 1;
const LEARNING_RATE: f64 = 3.6;
const EPSILON: f32 = 0.2;
const GAMMA: f32 = 0.94;
const MODEL_WEIGHTS_PATH:&str = "model.safetensors";

pub fn train() {

   
    // let device = Device::cuda_if_available(0).unwrap()
    let device = Device::Cpu;

    let mut varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = ConnectFourNN::new(vb, device.clone()).unwrap();
    varmap.load("model.safetensors");
    
    println!("Starting Training");
    

    let optimizer_config = ParamsAdamW {
        lr:LEARNING_RATE,
        ..ParamsAdamW::default()
    };
    let mut optimizer = AdamW::new(varmap.all_vars(), optimizer_config).unwrap();
    
    let model_config = ModelConfig { device, model };

    let training_config = TrainingConfig {
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
    };

    // for i in 0..NUM_BATCHES {
    //     //Training
        
    //     // let episode_iter = (0..BATCH_SIZE).into_par_iter().map(|_|run_episode(&model_config, &training_config));
    //     // let batch = Batch::from_par_iter(episode_iter);
    //     // batch.save(Path::new("episode_data"));


    //     // let avg_cost = optimize_model(batch,&model_config,&mut training_config);
    //     // var_map.save(MODEL_WEIGHTS_PATH);
    //     // println!("Batch: {}/{}, Avg Cost: {}",i,NUM_BATCHES,avg_cost);
    // }
    let batch = Batch::from(Path::new("episode_data\\Batch_04-17_18-37"));

    for i in 0..2 {
        // print_varmap_weights(&varmap);
        
        
        let batch = &batch;
        let avg_cost = optimize_model(batch,&mut optimizer, &model_config,&training_config);
        let _ = varmap.save(MODEL_WEIGHTS_PATH);

        println!("Batch: {}, Avg Cost: {}",i,avg_cost);

    }




}

pub struct ModelConfig {
    pub model: ConnectFourNN,
    pub device: Device,
}
pub struct TrainingConfig {
    pub epsilon: f32, // Exploration rate
    pub gamma: f32,   // Discount factor
}

impl GameOutcome {
    pub fn reward(&self) -> f32 {
        match &self {
            GameOutcome::Win => 0.1,
            GameOutcome::Tie => -0.005,
            GameOutcome::Loss => -0.1,
        }
    }
}

fn print_varmap_weights(varmap: &VarMap) {
    // Access the underlying data (HashMap<String, Var>)
    let data = varmap.data().lock().unwrap();
    
    // Iterate over each variable in the VarMap
    for (name, var) in data.iter() {
        // Get the tensor for this variable
        let tensor = var.as_tensor();
    
        
        // Convert tensor to a Vec<f32> for printing (only for small tensors or sample for large ones)
        let values = tensor.to_vec2::<f32>().unwrap_or_else(|_| vec![vec![]]); // Use to_vec2 for 2D, etc.
        
        if name.contains("bias") {
            continue;
        }
        // Print details
        println!("Parameter: {}", name);
        
        for i in values[0].iter().take(5) {
            print!(" {i},")
        }
        println!()
    }
    
    
}

