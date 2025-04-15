use std::path::Path;

use connect_four::player;
use training::{memory::episode_memory::EpisodeMemory, train::train};
use ui::spectate_game::spectate_game;

mod connect_four;
mod training;
mod ui;

fn main() {
    // let episode = EpisodeMemory::from_file_path(Path::new("episode_data\\Batch_2025-04-15_12-11-28\\Episode_3.csv"));
    // spectate_game(episode);
    train();
}
