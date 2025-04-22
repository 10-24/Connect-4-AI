use burn::{
    backend::{self, wgpu::WgpuDevice},
    tensor::Device,
};
use connect_four::player;
use std::path::Path;
use training::{
    memory::{batch::Batch, episode_memory::EpisodeMemory},
    model::model::Model,
    train::train,
};
use ui::{spectate_game::spectate_game, ui::create_game_window};
mod connect_four;
mod logger;
mod toy_nn;
mod training;
mod ui;

pub type StrippedBknd = backend::Wgpu<f32,i32>;
pub type Bknd = backend::Autodiff<StrippedBknd>;
pub const DEVICE:WgpuDevice = WgpuDevice::DefaultDevice;
pub type ModelWithBackend = Model<Bknd>;
pub type ModelWithStrippedBackend = Model<StrippedBknd>;

// pub type StrippedBknd = burn_cuda::Cuda;
// pub type Bknd = backend::Autodiff<StrippedBknd>;
// pub const DEVICE: CudaDevice = CudaDevice { index: 0 };
// pub type ModelWithBackend = Model<Bknd>;
// pub type ModelWithStrippedBackend = Model<StrippedBknd>;

fn main() {
    // let path = Path::new("episode_data\\Batch_04-21_20-34");
    // let batch = Batch::from_file(path);
    // let episode = EpisodeMemory::from_batch(batch).pop().unwrap();
    // spectate_game(episode);

    train();
    // create_game_window();
}
