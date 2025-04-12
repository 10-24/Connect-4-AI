use candle_core::{Device, Tensor};
use candle_nn::Module;
use derive_new::new;

use crate::{
    connect_four::{connect_four::{ConnectFour}, connect_four_enums::GameOutcome},
    player::Player,
};

use super::model::ConnectFourNN;

pub struct EpisodeSetup {
    pub model: ConnectFourNN,
    pub epsilon: f32,
    pub device: Device,
}

pub fn run_episode(setup: &EpisodeSetup) -> EpisodeResult {
    let mut memory = Memory::new();

    let mut game = ConnectFour::new(3);
    let mut rounds = 0;

    
    loop {
        // Player Blue
        let game_state_blue = game.get_board_blue_perspective();

        let player_blue_action = get_action(&setup.model, &game_state_blue);
        let selected_col = player_blue_action;

        memory.record(game_state_blue, player_blue_action as f32, &setup.device);

        let game_result_option = game.play_turn(selected_col);
        if let Some(game_result) = game_result_option {
            return EpisodeResult::new(rounds, game_result, memory);
        }

        // Player Red
        let game_state_red = game.get_board_red_perspective();
        let selected_col_red = get_action(&setup.model, &game_state_red);

        let game_result_option = game.play_turn(selected_col_red as usize);
        if let Some(game_result) = game_result_option {
            if game_result == GameOutcome::Win {
                return EpisodeResult::new(rounds, GameOutcome::Loss, memory);
            }
            return EpisodeResult::new(rounds, GameOutcome::Tie, memory);
        }

        rounds += 1;
    }
}

fn get_action(model: &ConnectFourNN, state: &Tensor) -> usize {
    let q_vals = model.forward(&state).unwrap();

    let q_vals_vec: Vec<f32> = q_vals.to_vec1().unwrap();

    let (col, _) = q_vals_vec
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    col
}

#[derive(new)]
pub struct EpisodeResult {
    pub rounds: u8,
    pub outcome: GameOutcome,
    pub memory: Memory,
}

pub struct Memory {
    pub game_states: Vec<GameState>,
    pub actions: Vec<Tensor>,
}
impl Memory {
    fn new() -> Self {
        Self {
            game_states: Vec::with_capacity(12),
            actions: Vec::with_capacity(12),
        }
    }
    fn record(&mut self, state: GameState, action: f32, device: &Device) {
        let action_array = [action];
        let action_tensor = Tensor::from_slice(action_array.as_slice(), (1, 1), device).unwrap();

        self.actions.push(action_tensor);
        self.game_states.push(state)
    }
}
