
use std::path::Path;
use burn::{backend::{self, wgpu::WgpuDevice}, tensor::Device};
use connect_four::player;

use training::model::model::Model;
use ui::spectate_game::spectate_game;
mod connect_four;
mod logger;
mod toy_nn;
mod training;
mod ui;

pub type StrippedBknd = backend::Wgpu<f32,i16>;
pub type Bknd = backend::Autodiff<StrippedBknd>;
pub const DEVICE:WgpuDevice = WgpuDevice::DefaultDevice;
pub type ModelWithBackend = Model<Bknd>;
pub type ModelWithStrippedBackend = Model<StrippedBknd>;

fn main() {
    
    // let path = "episode_data\\Batch_2025-04-17_07-38-29\\Episode_0.csv";
    // let episode = EpisodeMemory::from_path(Path::new(path));
    // spectate_game(episode);
    train();
}
