use candle_core::{shape, Device, Tensor};

use crate::connect_four::connect_four::ConnectFour;

use super::episode::Memory;

pub struct TrainingTensors {
    states: Tensor,
    actions: Tensor,
}
impl TrainingTensors {
    fn new(states_vec: Vec<Tensor>, actions_vec: Vec<f32>, device: &Device) -> Self {
        Self {
            states: Tensor::from_vec(states_vec, (states_vec.len(), ConnectFour::COLS), device).unwrap(),
            actions: Tensor::from_vec(actions_vec,(actions_vec.len(),1),&device).unwrap(),
        }
    }
}
