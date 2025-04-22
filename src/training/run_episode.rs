use crate::{
    connect_four::{connect_four::ConnectFour, game_board::GameBoard},
    Bknd, ModelWithBackend, DEVICE,
};
use burn::tensor::Tensor;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, Rng};

use super::{memory::episode_memory::EpisodeMemory, train::TrainingConfig};

pub fn run_episode(model: &ModelWithBackend, training_config: &TrainingConfig) -> EpisodeMemory {
    let mut rng: ThreadRng = rand::thread_rng();
    let episode_id = rng.gen();
    let mut memory = EpisodeMemory::new(episode_id);

    let mut game = ConnectFour::new(3);

    loop {
        let game_state = game.board.as_tensor(&game.current_player).unsqueeze();

        let selected_col = get_action(model, game_state, &mut rng, training_config);
        memory.record_frame(game.current_player, selected_col);

        let turn_result_option = game.play_turn(selected_col);
        if turn_result_option.is_some() {
            return memory;
        }

        game.current_player.switch();
    }
}

fn get_action(
    model: &ModelWithBackend,
    state: Tensor<Bknd, 1>,
    rng: &mut ThreadRng,
    training_config: &TrainingConfig,
) -> usize {
    let q_vals_t = model.forward(state.unsqueeze()).squeeze(1); 
    let col = sample_dist_random(q_vals_t,rng,training_config);
    col
}

fn sample_dist_random(normalized_q_vals_t:Tensor<Bknd,1>,rng:&mut ThreadRng, training_config:&TrainingConfig) -> usize {

    let output_t_shape = normalized_q_vals_t.shape();
    let epsilon = training_config.epsilon;
  

    let mean = 1.0 / (GameBoard::COLS as f32);
    let mean_x_epsilon_complement = mean * (1.0 - epsilon);
    let mean_x_epsilon_complement_t = Tensor::full(output_t_shape.clone(),mean_x_epsilon_complement,&DEVICE);

    let epsilon_t = Tensor::full(output_t_shape,epsilon,&DEVICE);
    let normalized_q_vals_x_epsilon_t = normalized_q_vals_t.mul(epsilon_t);

    let dist_t = mean_x_epsilon_complement_t.add(normalized_q_vals_x_epsilon_t);
    let dist: Vec<f32> = dist_t.to_data().to_vec().unwrap();
    let dist= WeightedIndex::new(dist.as_slice()).unwrap();
    dist.sample(rng)
}
