use burn::{
    config::Config,
    module::Module,
    nn::{
        loss::{MseLoss, Reduction},
        Linear, LinearConfig, Relu,
    },
    prelude::Backend,
    tensor::{Device, Int, Tensor},
    train::{RegressionOutput, TrainOutput, TrainStep},
};

use crate::{training::memory::batch::Batch, Bknd, DEVICE};

use super::target_q_val_builder::{self, TargetQValBuilder};
#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    pub layer_0: Linear<B>,
    pub layer_1: Linear<B>,
    pub layer_2: Linear<B>,
    pub activation: Relu,
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let xs = self.layer_0.forward(input);
        let xs = self.activation.forward(xs);

        let xs = self.layer_1.forward(xs);
        let xs = self.activation.forward(xs);

        let xs = self.layer_2.forward(xs);
        xs
    }
    pub fn forward_regression(
        &self,
        state: Tensor<B, 2>,
        target_q_val_builder: TargetQValBuilder<B>,
    ) -> RegressionOutput<B> {
        let predicted_q_vals = self.forward(state);
        let target_q_vals =
        target_q_val_builder.build(predicted_q_vals.clone(), &predicted_q_vals.device());
  
        let loss = MseLoss::new().forward(
            predicted_q_vals.clone(),
            target_q_vals.clone(),
            Reduction::Mean,
        );

        RegressionOutput::new(loss, predicted_q_vals, target_q_vals)
    }
}

impl TrainStep<Batch, RegressionOutput<Bknd>> for Model<Bknd> {
    fn step(&self, batch: Batch) -> TrainOutput<RegressionOutput<Bknd>> {
        let item =
            self.forward_regression(batch.get_game_states(), batch.create_target_q_val_builder());

        TrainOutput::new(self, item.loss.backward(), item)
    }
}
