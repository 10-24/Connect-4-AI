use burn::{config::Config, nn::{LinearConfig, Relu}, prelude::Backend};

use super::model::Model;

#[derive(Config, Debug)]
pub struct ModelConfig {
    // num_classes: usize,
    // hidden_size: usize,
    // #[config(default = "0.5")]
    // dropout: f64,
}

impl ModelConfig {

    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {

        let layer_0 = LinearConfig::new(42, 64).with_bias(true).init(device);
        let layer_1 = LinearConfig::new(64, 30).with_bias(true).init(device);
        let layer_2 = LinearConfig::new(30, 7).with_bias(true).init(device);
        let activation = Relu::new();

        Model { layer_0, layer_1, layer_2, activation, }
    }
}