// use std::path::Path;

// use candle_core::{DType, IndexOp, Tensor};
// use candle_nn::{linear, AdamW, Linear, Module, Optimizer, VarBuilder, VarMap};

// use crate::{connect_four::player::Player, training::memory::episode_memory::EpisodeMemory};

// fn toy_nn(){

//     let rewards =[-0.6240322,-0.68574965,-0.75357103,-0.8281,-0.91,-1.0];
//     let selected_cols = [1,5,2,4,5,0];


//     let device = candle_core::Device::Cpu;
//     let varmap = VarMap::new();
//     let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
//     let model = ToyNN::new( vb).unwrap();
//     let mut optimizer = AdamW::new(varmap.all_vars(), Default::default());
//     let game_states = EpisodeMemory::from_path(Path::new("episode_data\\Batch_04-17_18-37\\Episode_0.csv")).get_game_states(&Player::Blue, &device);

//     for epoch in 0..32 {
//         // forward pass 
//         let output = model.forward(&game_states).unwrap();

//         let rewards = Tensor::new(rewards.as_slice(),&device).unwrap().unsqueeze(1).unwrap();
//         let selected_cols = Tensor::new(selected_cols.as_slice(),&device).unwrap().unsqueeze(1).unwrap();
//         let target = output.scatter_add(indexes, source, dim)
//         let loss = candle_nn::loss::mse(&output, y_train)?;
//         output.i(index)
//         // backward pass and optimization
//         optimizer.backward_step(&loss)?;
        
//         if (epoch+1) % 10 == 0{
//             println!("Train Loss: {}", loss.to_scalar::<f32>()?);
//         } 
//     }
// }
// #[derive(Debug)]
// struct ToyNN{
//     fc1: Linear,
//     fc2: Linear,
// }

// // This function instantiates a new Model 
// impl ToyNN{
//     fn new(vb:VarBuilder)->candle_core::Result<Self>{
//         let fc1 = linear(42, 40, vb.pp("fc1"))?;
//         let fc2 = linear(40, 7, vb.pp("fc2"))?;

//         Ok(
//             Self { 
//                 fc1, fc2 
//             }
//         )
//     }
// }

// // forward pass of our model using Module trait 
// impl Module for ToyNN{
//     fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
//         let x = self.fc1.forward(xs)?.relu()?;
//         let x = self.fc2.forward(&x)?;

//         Ok(x)
//     }
// }