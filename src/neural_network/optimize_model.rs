use candle_core::{Device, Shape, Tensor};
use candle_nn::{loss, Module};
use derive_new::new;

use crate::connect_four::connect_four_enums::GameOutcome;

use super::{episode::{Action, EpisodeResult, MemoryFrame}, model::ConnectFourNN, train::{ModelConfig, TrainingConfig}};

pub fn optimize_model(result: &EpisodeResult, model_config: &ModelConfig,training_config:&TrainingConfig) {

    
    let ModelConfig { model,device,..}= model_config;

    let MemoryFrame { action, state:game_state } = result.memory;

        let Action {
            q_values: expected_q_vals_tensor,
            col: selected_col,
        } = action;

        let expected_q_vals = model.forward(game_state).unwrap().gather(selected_col, 1).unwrap();
        
        let target_rewards = generate_target_rewards(result.outcome,model_config,training_config);
        let target_q_vals = expected_q_vals.clone().scatter_add(indexes, source, 1);
        let cost = loss::mse(&expected_q_vals, &target_q_vals).unwrap();
        
        actual_reward *= gamma;
}

fn adjust_weights(model_config: &ModelConfig,){
    let MemoryFrame { action, state } = memory_frame;
    model.forward(&state).unwrap().gather(indexes, dim);
    
}


fn generate_target_rewards(game_outcome:GameOutcome,model_config: &ModelConfig, training_config:&TrainingConfig) -> Tensor {
   
    let ModelConfig {
        device,
        ..
    } = model_config;

    let TrainingConfig { gamma,batch_size,..} = training_config;

    let reward_vec_len = batch_size.clone() as usize;
    
    
    let terminal_state_value = game_outcome.reward();
    
    let mut target_state_values: Vec<f32> = vec![0.; reward_vec_len];
    let mut current_state_value = terminal_state_value;
    for i in target_state_values.iter_mut().rev() {
        *i = current_state_value.clone();
        current_state_value *= gamma;
    }
    
    Tensor::from_vec(target_state_values, (reward_vec_len,1), device).unwrap()
}
