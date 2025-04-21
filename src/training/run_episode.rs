use std::f32;

use crate::{
    connect_four::{
        connect_four::ConnectFour, connect_four_enums::Outcome, game_board::GameBoard,
        player::Player,
    },
    Bknd, ModelWithBackend,
};
use burn::tensor::Tensor;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, Rng};

use super::{
    memory::episode_memory::{EpisodeMemory, GameFrame},
    train::TrainingConfig,
};

pub fn run_episode(model: &ModelWithBackend, training_config: &TrainingConfig) -> EpisodeMemory {
    let mut rng: ThreadRng = rand::thread_rng();
    let episode_id = rng.gen();
    let mut memory = EpisodeMemory::new(episode_id);

    let mut game = ConnectFour::new(3);

    loop {
        let game_state = game.board.as_tensor(&game.current_player).unsqueeze();

        let selected_col = get_action(&model, game_state, &mut rng, training_config);

        memory.record_turn(game.current_player, selected_col);

        let turn_result_option = game.play_turn(selected_col);
        if turn_result_option.is_some() {
            return memory;
        }

        game.current_player.switch();
    }
}

fn get_action(
    model: &ModelWithBackend,
    state: Tensor<Bknd, 2>,
    rng: &mut ThreadRng,
    training_config: &TrainingConfig,
) -> usize {
    let q_vals_t = model.forward(state);

    // let normalized_q_vals_t = softmax(&q_vals_t, 1).unwrap();
    // let col = sample_dist_random(&normalized_q_vals_t,rng,model_config,training_config);
    if rng.gen_bool(training_config.epsilon as f64) {
        return q_vals_t.argmax(1).into_scalar() as usize;
    }
    rng.gen_range(0..GameBoard::COLS)
}

// fn _sample_dist_random(normalized_q_vals_t:&Tensor,rng:&mut ThreadRng, model_config:&ModelConfig,training_config:&TrainingConfig) -> usize {

//     let output_t_shape = normalized_q_vals_t.shape();
//     let epsilon = training_config.epsilon;
//     let device = &model_config.device;

//     let mean = 1.0 / (GameBoard::COLS as f32);
//     let mean_x_epsilon_complement = mean * (1.0 - epsilon);
//     let mean_x_epsilon_complement_t = Tensor::full(mean_x_epsilon_complement,output_t_shape,device).unwrap();

//     let epsilon_t = Tensor::full(epsilon,output_t_shape,device).unwrap();
//     let normalized_q_vals_x_epsilon_t = normalized_q_vals_t.mul(&epsilon_t).unwrap();

//     let dist_t = mean_x_epsilon_complement_t.add(&normalized_q_vals_x_epsilon_t).unwrap();
//     let dist_v= &dist_t.to_vec2::<f32>().unwrap()[0];

//     let weighted_dist = WeightedIndex::new(dist_v).unwrap();

//     weighted_dist.sample(rng)
// }
