use candle_ext::TensorExt;
use candle_core::Tensor;
use crate::{connect_four::player::Player, training::{memory::episode_memory::EpisodeMemory, train::{ModelConfig, TrainingConfig}}};



pub fn generate_target_rewards(
    episode: &EpisodeMemory,
    model_config: &ModelConfig,
    training_config: &TrainingConfig,
) -> Tensor {
    let rewards_v = calculate_rewards(&episode,&training_config);
    // let rewards_t = Te
    todo!()
}

fn calculate_rewards(episode:&EpisodeMemory,training_config: &TrainingConfig) -> Vec<f32>{
    let num_turns = episode.number_of_turns(&Player::Blue);
    let gamma = training_config.gamma;
    let terminal_state_reward = episode.outcome(&Player::Blue).reward();

    let mut target_values = vec![f32::default(); num_turns];
    let mut current_state_value = terminal_state_reward;

    for i in target_values.iter_mut().rev() {
        *i = current_state_value;
        current_state_value *= gamma;
    }
    target_values
}