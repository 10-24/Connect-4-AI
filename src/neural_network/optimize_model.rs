use candle_core::Tensor;
use candle_nn::{loss, Module, Optimizer};

use crate::{connect_four::connect_four_enums::GameOutcome, BATCH_SIZE};

use super::{episode::EpisodeResult, train::{ModelConfig, TrainingConfig}};

pub fn optimize_model(episode_results: Vec<EpisodeResult>, model_config: &ModelConfig,training_config:&mut TrainingConfig) {

    let model = &model_config.model;

    for episode_result in episode_results {

        let memory = &episode_result.memory;
        
        let game_states = memory.get_game_states();
        let expected_q_vals = model.forward(&game_states).unwrap();
        println!("expected_q_vals: {:?}",expected_q_vals.to_vec2::<f32>().unwrap());

        let selected_cols = memory.get_actions();

        let target_rewards = generate_target_rewards(&episode_result.outcome, model_config, training_config);
        let target_q_vals = expected_q_vals.clone()
            .scatter_add(&selected_cols, &target_rewards, 1).unwrap();
        println!("target_q_vals: {:?}",target_q_vals.to_vec2::<f32>().unwrap());

        let loss = loss::mse(&expected_q_vals, &target_q_vals).unwrap();

        println!("loss: {:?}",loss.to_vec2::<f32>().unwrap());
      
        training_config.optimizer.backward_step(&loss);

    }
}




fn generate_target_rewards(game_outcome:&GameOutcome,model_config: &ModelConfig, training_config:&TrainingConfig) -> Tensor {
   

    let device = &model_config.device;
    let gamma = training_config.gamma;
    let terminal_state_value = game_outcome.reward();
    
    
    let mut target_state_values = [f32::default(); BATCH_SIZE];
    let mut current_state_value = terminal_state_value;
    for i in target_state_values.iter_mut().rev() {
        *i = current_state_value;
        current_state_value *= gamma;
    }
    
    Tensor::from_slice(target_state_values.as_slice(), target_state_values.len(), device).unwrap()
}
