use candle_core::{DType, Device};
use candle_nn::{AdamW, Optimizer, ParamsAdamW, VarBuilder, VarMap};
use neural_network::{
    model::ConnectFourNN,
    train::{train, ModelConfig, TrainingConfig},
};

mod connect_four;
mod neural_network;
mod ui;

mod player;

fn main() {
    // ui::create_window();

    const LEARNING_RATE: f64 = 0.0001;
    const EPSILON: f32 = 0.88;
    const GAMMA: f32 = 0.91;
    const NUM_BATCHES: u16 = 5;
    const BATCH_SIZE: u16 = 5;
    let device = Device::cuda_if_available(0).unwrap();

    let var_map = VarMap::new();
    let vb = VarBuilder::from_varmap(&var_map, DType::F32, &device);
    let optimizer_config = ParamsAdamW {
        lr: LEARNING_RATE,
        ..Default::default()
    };
    let optimizer = AdamW::new(var_map.all_vars(), optimizer_config).unwrap();

    let model = ConnectFourNN::new(vb, device.clone()).expect("unable to init nn");

    let model_config = ModelConfig { device, model };

    let training_config = TrainingConfig {
        optimizer,
        batch_size: BATCH_SIZE,
        num_batches: NUM_BATCHES,
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
    };

    train(model_config, training_config);
}
