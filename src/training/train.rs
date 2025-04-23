use crate::{
    training::{
        memory::convert_episode_to_training_frames::convert_episode_to_training_frames,
        model::model_config::ModelConfig, optimize_model::optimize_model::optimize_model,
    },
    ModelWithBackend, DEVICE,
};
use burn::{
    module::Module,
    optim::AdamConfig,
    record::{DefaultFileRecorder, FullPrecisionSettings, NamedMpkFileRecorder, Recorder},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::{training::{memory::convert_episode_to_training_frames::convert_episode_to_training_frames, model::model_config::ModelConfig, optimize_model::optimize_model::optimize_model}, ModelWithBackend, DEVICE};

use super::{memory::batch::{Batch, BatchFile}, run_episode::run_episode};

pub const NUM_BATCHES: u16 = 1000;
pub const BATCH_SIZE: usize = 6;
pub const NUM_BATCHES: u16 = 3;
pub const BATCH_SIZE: usize = 5;
pub const SAVE_INTERVAL:u16 = 130;

const EPSILON: f32 = 0.65;
const GAMMA: f32 = 0.6;
const MODEL_WEIGHTS_PATH: &str = "model.safetensors";


pub fn train() {
    let model_parameters_recorder: NamedMpkFileRecorder<FullPrecisionSettings> =
    DefaultFileRecorder::new();
    let mut model = ModelConfig::new().init(&DEVICE);
    
    let training_config = TrainingConfig {
        epsilon: EPSILON, // Exploration rate
        gamma: GAMMA,     // Discount factor
        learning_rate: 0.0001,
    };
    let mut optimizer = AdamConfig::new().init();
    println!("Starting Training");
    let mut batch_stats = [TrainingStepStats::default();NUM_BATCHES as usize];
    let mut m = 0;    let mut batch_file = BatchFile::new("episode_data");

    for i in 1..=NUM_BATCHES {
        m+=1;
        println!("{m}");
        let start_time = Utc::now().timestamp_millis();
        let episodes = (0..BATCH_SIZE).into_par_iter()
        .map(|_| {
            let model = ModelConfig::new().init(&DEVICE);
            run_episode(&model, &training_config)
        });
        
        m+=1;
        println!("{m}");
        let training_frames = episodes
        .map(|episode| convert_episode_to_training_frames(episode, &training_config))
        .reduce(Vec::new, |mut reducer, mut val| {
            reducer.append(&mut val);
            reducer
        });
        let batch = Batch::new(training_frames);

        let (new_model, loss) =
            optimize_model(batch.clone(), model, &mut optimizer, &training_config);

            model = new_model;
        println!("batch {i}/{NUM_BATCHES}, loss {loss}");
        batch_file.add(batch);
    }
    
    batch_file.save();
    model.save_file(MODEL_WEIGHTS_PATH, &model_parameters_recorder).expect("Unable to save model");
   

        batch.save(Path::new("episode_data"),i.to_string().as_str());
        
        let (new_model,loss) = optimize_model(batch.clone(),model,&mut optimizer,&training_config,);
        model = new_model;
        if i % SAVE_INTERVAL == 0 {
            model.clone().save_file(MODEL_WEIGHTS_PATH, &model_parameters_recorder).expect("Cannot save model parameters");
        }
        let end_time = Utc::now().timestamp_millis();
        let duration_ms = end_time-start_time;
        println!("batch {i}/{NUM_BATCHES}, loss {}, time {duration_ms}ms",1000.0*loss);
        m+=1;
        println!("{m}");
        batch_stats[i as usize - 1] = TrainingStepStats::new(loss, duration_ms as u16);
    }
    print_training_summary(batch_stats);
    
}

#[derive(Clone, Copy)]
pub struct TrainingConfig {
    pub epsilon: f32,
    pub gamma: f32,
 
    pub learning_rate: f32,
}

