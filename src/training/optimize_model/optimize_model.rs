use candle_core::Tensor;
use candle_ext::{TensorExt,};

use candle_nn::{loss, AdamW, Module, Optimizer, SGD};
use egui::TextBuffer;

use crate::{
    connect_four::{connect_four_enums::GameOutcome, game_board::GameBoard, player::Player},
    logger::Logger, training::{memory::batch::Batch, optimize_model::generate_target_rewards::generate_target_rewards, train::{ModelConfig, TrainingConfig}},
};


pub fn optimize_model(
    batch: &Batch,
    optimizer: &mut AdamW,
    model_config: &ModelConfig,
    training_config: &TrainingConfig,
) -> f32 {
    let model = &model_config.model;
    let mut loss_sum: f32 = 0.0;

    for episode in batch.episodes.iter() {
        
        let game_states = episode.get_game_states(&Player::Blue, &model_config.device);
        let selected_cols = episode.get_actions(&Player::Blue, &model_config.device);

        let output_q_vals = model.forward(&game_states).unwrap();

        let target_rewards = generate_target_rewards(episode, &model_config, &training_config);
        target_rewards.scatter(&ids, &t, 1);
        let target_q_vals = replace_indexes(
            &output_q_vals,
            &selected_cols.squeeze(1).unwrap(),
            &target_rewards,
        )
        .unwrap();


        let loss = candle_nn::loss::mse(&target_q_vals, &output_q_vals).unwrap();

        let _ = optimizer.backward_step(&loss);
        println!(
            "Q-Val Mean: {}, Q-Val Std: {}",
            output_q_vals
                .mean_all()
                .unwrap()
                .to_scalar::<f32>()
                .unwrap(),
            tensor_std(&output_q_vals)
        );
        loss_sum += loss.to_scalar::<f32>().unwrap();
    }
    loss_sum / batch.episodes.len() as f32
}




fn tensor_std(tensor: &Tensor) -> f32 {
    let mean = tensor.mean_all().unwrap();
    let squared_diff = tensor.broadcast_sub(&mean).unwrap().sqr().unwrap();
    let mean_squared_diff = squared_diff.mean_all().unwrap().to_scalar::<f32>().unwrap();
    mean_squared_diff.sqrt()
}

