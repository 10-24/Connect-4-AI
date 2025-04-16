use candle_core::Tensor;
use candle_nn::{loss, Module, Optimizer};

use crate::connect_four::{connect_four_enums::GameOutcome, player::Player};

use super::{memory::{batch::Batch, episode_memory::EpisodeMemory}, train::{ModelConfig, TrainingConfig, BATCH_SIZE}};

pub fn optimize_model(batch: Batch, model_config: &ModelConfig,training_config:&mut TrainingConfig) -> f32 {

    let model = &model_config.model;
    let mut loss_sum: f32 = 0.0;
    for memory in batch.episodes {

        let game_states = memory.get_game_states(&Player::Blue, &model_config.device);
        let selected_cols = memory.get_actions(&Player::Blue, &model_config.device);

        let expected_q_vals = model.forward(&game_states).unwrap();
    
        let target_rewards = generate_target_rewards(&memory, model_config, training_config);

        let target_q_vals = expected_q_vals.clone()
            .scatter_add(&selected_cols, &target_rewards, 1).unwrap();

        let loss = loss::mse(&expected_q_vals, &target_q_vals).unwrap();
        training_config.optimizer.backward_step(&loss);
        
        loss_sum += loss.to_scalar::<f32>().unwrap();
    }
    loss_sum / BATCH_SIZE as f32
}




fn generate_target_rewards(memory:&EpisodeMemory,model_config: &ModelConfig, training_config:&TrainingConfig) -> Tensor {
   
    let num_turns = memory.number_of_turns(&Player::Blue);
    let device = &model_config.device;
    let gamma = training_config.gamma;
    let terminal_state_reward = memory.outcome(&Player::Blue).reward();
    
    let mut target_state_values = vec![f32::default();num_turns];
    let mut current_state_value = terminal_state_reward;
    for i in target_state_values.iter_mut().rev() {
        *i = current_state_value;
        current_state_value *= gamma;
    }
    
    Tensor::new(target_state_values.as_slice(), device).unwrap().unsqueeze(1).unwrap()
}

