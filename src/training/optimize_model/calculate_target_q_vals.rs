use burn::{
    backend::wgpu::BoolElement,
    tensor::{Bool, Int, Shape, Tensor, TensorData},
};
use num_traits::Pow;

use crate::{
    connect_four::{connect_four_enums::Outcome, game_board::GameBoard, player::Player},
    training::{
        memory::episode_memory::EpisodeMemory,
        optimize_model::tensor_ops::{create_mask, mask_negative},
        train::TrainingConfig,
    },
    Bknd, DEVICE,
};

pub fn create_target_q_vals(
    outcome: &Outcome,
    predicted_q_vals: Tensor<Bknd, 2>,
    selected_cols: Tensor<Bknd, 1, Int>,

    training_config: &TrainingConfig,
) -> Tensor<Bknd, 2> {
    let target_q_vals_shape = predicted_q_vals.shape();
    let num_turns = selected_cols.shape().num_elements();

    // todo remove clones
    let selected_col_mask = create_mask(selected_cols.clone(), target_q_vals_shape.clone());

    let rewards_1d_vec = calculate_rewards(outcome, num_turns, training_config.gamma);
    let rewards_1d: Tensor<_, 1> = Tensor::from(rewards_1d_vec.as_slice());
    let rewards_2d_full = rewards_1d
        .unsqueeze_dim::<2>(1)
        .expand(target_q_vals_shape.clone());

    predicted_q_vals.mask_where(selected_col_mask, rewards_2d_full)
}
fn calculate_rewards(outcome: &Outcome, num_turns: usize, gamma: f32) -> Vec<f32> {
    let mut rewards = vec![f32::default(); num_turns];

    let terminal_state_reward = outcome.reward();
    let terminal_state_index = (num_turns - 1) as i32;
    for (i, val) in rewards.iter_mut().enumerate() {
        *val = terminal_state_reward * gamma.powi(terminal_state_index - i as i32);
        // Could be more efficent
    }
    rewards
}
