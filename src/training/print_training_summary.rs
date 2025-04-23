use burn::tensor::ElementComparison;
use derive_new::new;

use super::train::{NUM_BATCHES,BATCH_SIZE};

pub fn print_training_summary(stats:[TrainingStepStats;NUM_BATCHES as usize]){
    
    println!("Training Complete!");
    println!("Batches: {NUM_BATCHES}, Episodes per Batch {BATCH_SIZE}");

    let (min_loss_index,min_loss_training_step) = stats.iter().enumerate().min_by(|a,b| a.1.loss.cmp(&b.1.loss)).unwrap();
    let min_loss = min_loss_training_step.loss;
    let min_loss_percentage = min_loss_index as f32 /NUM_BATCHES as f32;
    println!("Min Loss: {min_loss} @ {min_loss_percentage}%");

    let avg_duration:u32 = stats.iter().map(|i|i.duration_ms as u32).sum::<u32>() / NUM_BATCHES as u32;
    println!("Avg Batch Duration: {avg_duration}");

    
}

#[derive(Default,new,Clone, Copy)]
pub struct TrainingStepStats {
    loss:f32,
    duration_ms:u16,
}
