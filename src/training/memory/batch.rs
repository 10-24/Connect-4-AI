use csv::{Error, Reader};
use rayon::iter::{FromParallelIterator, ParallelIterator};
use serde::Serialize;
use std::{fs::File, io::Read, path::Path};

use crate::{
    connect_four::player::Player,
    training::{memory::episode_memory::GameTurn, train::BATCH_SIZE},
};
use chrono::Local;

use super::episode_memory::{self, EpisodeMemory};
use std::fs;

pub struct Batch {
    pub episodes: Vec<EpisodeMemory>,
}
impl Batch {
    pub fn new() -> Self {
        Self {
            episodes: Vec::with_capacity(BATCH_SIZE),
        }
    }
    pub fn from_par_iter<I>(iter: I) -> Self
    where
        I: ParallelIterator<Item = EpisodeMemory>,
    {
        Self {
            episodes: Vec::from_par_iter(iter),
        }
    }
    pub fn add(&mut self, new_memory: EpisodeMemory) {
        self.episodes.push(new_memory);
    }

    pub fn save(&self, folder: &Path) {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let batch_folder_name = format!("Batch_{timestamp}");
        let batch_folder = folder.join(batch_folder_name);
        fs::create_dir(batch_folder.clone());

        for (i, memory) in self.episodes.iter().enumerate() {
            let file_name = format!("Episode_{i}.csv");
            let path = batch_folder.join(file_name);
            memory.save(&path);
        }
    }

    pub fn from_folder(batch_folder: &Path) -> Self {
        let mut episodes = Vec::with_capacity(BATCH_SIZE);
        let dir = fs::read_dir(batch_folder).unwrap();
        for entry in dir.into_iter() {
            let entry = entry.unwrap();
            let new_episode = EpisodeMemory::from_file_path(&entry.path());

            episodes.push(new_episode);
        }

        Self { episodes }
    }
}
