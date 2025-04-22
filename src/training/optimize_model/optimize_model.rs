use burn::{
   optim::{adaptor::OptimizerAdaptor, Adam,  GradientsParams, Optimizer},
};
use crate::{
    training::{
        memory::batch::Batch,
        train::TrainingConfig,
    },
    Bknd, ModelWithBackend,
};

pub fn optimize_model(
    batch: Batch,
    model: ModelWithBackend,
    optimizer: &mut OptimizerAdaptor<Adam, ModelWithBackend, Bknd>,
    training_config: &TrainingConfig,
) -> (ModelWithBackend,f32) {

    let regression_output = model.forward_regression(batch.get_game_states(),batch.create_target_q_val_builder());
    let gradients = GradientsParams::from_grads(regression_output.loss.backward(),&model);
   
    let new_model = optimizer.step(training_config.learning_rate.into(), model, gradients);
    (new_model,regression_output.loss.into_scalar())
}
