use std::f32;

use super::{
    model::ConnectFourNN,
    train::{ModelConfig, TrainingConfig},
};
use crate::connect_four::{connect_four::ConnectFour, connect_four_enums::GameOutcome};
use candle_core::{Device, Tensor};
use candle_nn::Module;
use derive_new::new;

pub fn run_episode(model_config: &ModelConfig, _training_config: &TrainingConfig) -> EpisodeResult {
    let mut memory = Memory::new();

    let mut game = ConnectFour::new(3);

    let ModelConfig { model, device, .. } = model_config;
    loop {
        // Player Blue
        let game_state_blue = game.board.get_blue_perspective(device);

        let player_blue_action = get_action(model, &game_state_blue);
        let selected_col = player_blue_action;

        memory.record(game_state_blue, player_blue_action as f32, device);

        let game_result_option = game.play_turn(selected_col);
        if let Some(game_result) = game_result_option {
            return EpisodeResult::new(game_result, memory);
        }

        // Player Red
        let game_state_red = game.board.get_red_perspective(device);
        let selected_col_red = get_action(model, &game_state_red);

        let game_result_option = game.play_turn(selected_col_red);
        if let Some(game_result) = game_result_option {
            if game_result == GameOutcome::Win {
                return EpisodeResult::new(GameOutcome::Loss, memory);
            }
            return EpisodeResult::new(GameOutcome::Tie, memory);
        }
    }
}

fn get_action(model: &ConnectFourNN, state: &Tensor) -> usize {
    println!("game_state {:?}", state.to_vec2::<f32>().unwrap());
    println!("game_state len {}", state.to_vec2::<f32>().unwrap().len());
    let q_vals = model.forward(state).unwrap();

    let q_vals_vec: Tensor = q_vals.get(0).unwrap();
    let col: u32 = q_vals_vec.argmax(0).unwrap().to_vec0().unwrap();

    println!("selected col {col}");
    col as usize
}

#[derive(new, Default, Clone)]
pub struct EpisodeResult {
    pub outcome: GameOutcome,
    pub memory: Memory,
}
#[derive(Default, Clone)]
pub struct Memory {
    game_states: Vec<Tensor>,
    actions: Vec<Tensor>,
}
impl Memory {
    fn new() -> Self {
        Self {
            game_states: Vec::with_capacity(12),
            actions: Vec::with_capacity(12),
        }
    }
    fn record(&mut self, state: Tensor, action: f32, device: &Device) {
        let action_array = [action];
        let action_tensor = Tensor::from_slice(action_array.as_slice(), (1, 1), device).unwrap();

        self.actions.push(action_tensor);
        self.game_states.push(state)
    }
    pub fn get_game_states(&self) -> Tensor {
        Self::vec_to_tensor(&self.game_states)
    }
    pub fn get_actions(&self) -> Tensor {
        Self::vec_to_tensor(&self.actions)
    }
    fn vec_to_tensor(vec: &Vec<Tensor>) -> Tensor {
        Tensor::cat(vec.as_slice(), 0).unwrap()
    }
}
