use std::f32;
use candle_core::Tensor;
use super::{
    memory::episode_memory::EpisodeMemory,
    model::ConnectFourNN,
    train::{self, ModelConfig, TrainingConfig, BATCH_SIZE},
};
use crate::connect_four::{
    connect_four::ConnectFour, connect_four_enums::GameOutcome, game_board::GameBoard, player::Player
};
use candle_nn::{ops::softmax, Module};
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng};


pub fn run_episode(model_config: &ModelConfig, training_config: &TrainingConfig) -> EpisodeMemory {
    let mut memory = EpisodeMemory::new();
  
    let mut game = ConnectFour::new(3);

    let ModelConfig { model, device, .. } = model_config;
    let mut rng = rand::thread_rng();

    loop {
        let game_state = game.board.get_board_tensor(&game.current_player, device);

        let selected_col = get_action(&game_state, &mut rng,model_config,training_config);

        memory.record_turn(game.current_player, selected_col);

        let turn_result_option = game.play_turn(selected_col);
        if turn_result_option.is_some() {
            return memory;
        }

        game.current_player.switch();
    }
}

fn get_action(state: &Tensor,rng:&mut ThreadRng, model_config:&ModelConfig,training_config:&TrainingConfig,) -> u8 {
    let q_vals_t = model_config.model.forward(state).unwrap();
    let normalized_q_vals_t = softmax(&q_vals_t, 1).unwrap();
    let col = sample_dist(&normalized_q_vals_t,rng,model_config,training_config);
    col as u8
}
fn sample_dist(normalized_q_vals_t:&Tensor,rng:&mut ThreadRng, model_config:&ModelConfig,training_config:&TrainingConfig) -> usize {

    let output_t_shape = normalized_q_vals_t.shape();

    let epsilon = training_config.epsilon;
    let device = &model_config.device;

    
    let mean = 1.0 / (GameBoard::COLS as f32);
    let mean_x_epsilon_complement = mean * (1.0 - epsilon);
    let mean_x_epsilon_complement_t = Tensor::full(mean_x_epsilon_complement,output_t_shape,device).unwrap();
    
    let epsilon_t = Tensor::full(epsilon,output_t_shape,device).unwrap();
    let normalized_q_vals_x_epsilon_t = normalized_q_vals_t.mul(&epsilon_t).unwrap();
    
    let dist_t = mean_x_epsilon_complement_t.add(&normalized_q_vals_x_epsilon_t).unwrap();
    let dist_v= &dist_t.to_vec2::<f32>().unwrap()[0];
    
    let weighted_dist = WeightedIndex::new(dist_v).unwrap();

    weighted_dist.sample(rng)
}
