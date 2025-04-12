use candle_core::{DType, Device, Tensor};
use candle_nn::{AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use neural_network::{
    model::ConnectFourNN,
    train::{train, ModelConfig, TrainingConfig},
};
use ui::ui::create_window;

mod connect_four;
mod neural_network;
mod player;
mod ui;
pub const NUM_BATCHES: u16 = 1;
pub const BATCH_SIZE: usize = 1;
const LEARNING_RATE: f64 = 0.0001;
const EPSILON: f32 = 0.88;
const GAMMA: f32 = 0.91;
fn main() {
 
    let device = Device::cuda_if_available(0).unwrap();

    let var_map = VarMap::new();
    let vb = VarBuilder::from_varmap(&var_map, DType::F32, &device);
    let optimizer_config = ParamsAdamW {
        lr: LEARNING_RATE,
        ..Default::default()
    };
    
    let optimizer = AdamW::new(var_map.all_vars(), optimizer_config).unwrap();
    
    let model = ConnectFourNN::new(vb, device.clone()).expect("unable to init nn");

    let model_config = ModelConfig { device, model, num_data_type:DType::F32};

    let training_config = TrainingConfig {
        optimizer,
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
    };

    train(model_config, training_config);
}