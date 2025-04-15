use std::f32;

use super::{
    memory::episode_memory::EpisodeMemory,
    model::ConnectFourNN,
    train::{ModelConfig, TrainingConfig},
};
use crate::connect_four::{
    connect_four::ConnectFour, connect_four_enums::GameOutcome, player::Player,
};
use candle_core::{Device, Tensor};
use candle_nn::{ops::softmax, Module};
use derive_new::new;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, Rng};

pub fn run_episode(model_config: &ModelConfig, _training_config: &TrainingConfig) -> EpisodeMemory {
    let mut memory = EpisodeMemory::new();

    let mut game = ConnectFour::new(3);

    let ModelConfig { model, device, .. } = model_config;

    loop {
        let game_state = game.board.get_board_tensor(&game.current_player, device);

        let selected_col = get_action(model, &game_state);

        memory.record_turn(game.current_player, selected_col);

        let turn_result_option = game.play_turn(selected_col);
        if turn_result_option.is_some() {
            return memory;
        }

        game.current_player.switch();
    }
}

fn get_action(model: &ConnectFourNN, state: &Tensor) -> u8 {
    let q_vals = model.forward(state).unwrap();

    let q_vals_vec: Tensor = q_vals.get(0).unwrap();
    let col: u32 = q_vals_vec.argmax(0).unwrap().to_vec0().unwrap();

    col as u8
}
fn sample(tensor:&Tensor,gamma:f32,rng:&mut ThreadRng) -> usize {
    let tensor = softmax(tensor, 0).unwrap();
    let tensor_as_vec= tensor.to_vec1::<u32>().unwrap();
    let dist: WeightedIndex<u32> = WeightedIndex::new(tensor_as_vec).unwrap();
    dist.sample(rng)
}