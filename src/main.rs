use std::path::Path;

use connect_four::player;
use training::{memory::episode_memory::EpisodeMemory, train::train};
use ui::spectate_game::spectate_game;

mod connect_four;
mod training;
mod ui;

fn main() {
    // let path = "episode_data\\Batch_2025-04-17_07-38-29\\Episode_0.csv";
    // let episode = EpisodeMemory::from_path(Path::new(path));
    // spectate_game(episode);
    train();
}
