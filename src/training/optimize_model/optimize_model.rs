use burn::{
    optim::{adaptor::OptimizerAdaptor, Adam, GradientsAccumulator, GradientsParams, Optimizer},
    tensor::{self, Int, Tensor, TensorData}, train::TrainStep,
};
use serde::de::value;

use crate::{
    connect_four::{connect_four_enums::Outcome, game_board::GameBoard, player::Player},
    training::{
        run_episode, memory::batch::Batch,
        optimize_model::calculate_target_q_vals::create_target_q_vals, train::TrainingConfig,
    },
    Bknd, ModelWithBackend, DEVICE,
};

pub fn optimize_model(
    batch: Batch,
    model: ModelWithBackend,
    optimizer: &mut OptimizerAdaptor<Adam, Tensor<Bknd, 1>, Bknd>,
    training_config: &TrainingConfig,
) -> () {
    // for episode in batch.episodes.iter() {

    model.step( batch);
    let mut accumulator = GradientsAccumulator::new();
    let model_gradients= model.backward()
    let new_model = optimizer.step(training_config.learning_rate.into(), model, model_gradients);

    
}
