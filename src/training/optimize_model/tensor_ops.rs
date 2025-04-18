use anyhow::{Error, Ok};
use candle_core::{cpu_backend::CpuDevice, DType, Device, IndexOp, Tensor};
use derive_new::new;
use num_traits::ToPrimitive;

use crate::logger::Logger;

pub fn replace_indexes(src: &Tensor, cols: &Tensor, vals: &Tensor) -> anyhow::Result<Tensor> {

    let src_shape = src.shape();
    
    let zeros = Tensor::zeros(src_shape, DType::F32, &Device::Cpu)?;
    let result = zeros.scatter_add(&cols.unsqueeze(1)?.broadcast_as(src_shape)?, &vals.broadcast_as(src_shape)?.contiguous()?, 0)?;
    println!("result \n{}",result);
    
    // let neg_src = src
    //     .index_select(cols, 1)?
    //     .i(0)?
    //     .squeeze(0)?
    //     .unsqueeze(1)?
    //     .neg()?;

    // let addends = vals.add(&neg_src)?;

    // let expanded_cols = cols.unsqueeze(1)?.broadcast_as(src_shape)?.contiguous()?;
    // println!("src \n{}",src);

    // let new_tensor = src.scatter_add(&expanded_cols, &addends, 1)?;

    // // Logger::log_tensor_2d_f32("tensor_ops", ["src".into()].as_slice(), src);
    // // Logger::log_tensor_2d_f32("tensor_ops", ["reward".into()].as_slice(), vals);
    // // Logger::log_tensor_2d_f32("tensor_ops", ["addend".into()].as_slice(), &addends);
    // // Logger::log_tensor_2d_f32("tensor_ops", ["new_tensor".into()].as_slice(), &new_tensor);
    // print_comparison(&[
    //     PrintableMatrix::new::<u32>("cols  ", &expanded_cols),
    //     PrintableMatrix::new::<f32>("src   ", src),
    //     PrintableMatrix::new::<f32>("neg_  ", &neg_src),
    //     PrintableMatrix::new::<f32>("reward", vals),
    //     PrintableMatrix::new::<f32>("addend", &addends),
    //     PrintableMatrix::new::<f32>("new   ", &new_tensor),
    // ]);

    panic!("Must not pass");
    // Ok(new_tensor)

}

pub fn print_comparison(printable_tensors: &[PrintableMatrix]) {


    let all_same_len= printable_tensors.iter().all(|i| i.vec.len() == printable_tensors[0].vec.len());
    assert!(all_same_len, "all tensors must have the same number of rows");

    let names:Vec<String> = printable_tensors.iter().map(|i|i.name.clone()).collect::<Vec<String>>();
    println!("Comparing {} ------------------------",names.join(" & "));
    for i in 0..printable_tensors[0].vec.len(){
        
        println!("[{i}]");
        for named_tensor in printable_tensors {
            println!("{} {:?}",named_tensor.name,named_tensor.vec[i]);
        }
    }
    println!("--------------------------");
}

pub struct PrintableMatrix {
    name: String,
    vec: Vec<Vec<f32>>,
}
impl PrintableMatrix {
    pub fn new<T: ToPrimitive + candle_core::WithDType>(
        name: &str,
        tensor: &Tensor,
    ) -> Self {
        let tensor_as_vec_t = matrix_to_vec::<T>(tensor);
        let tensor_as_vec_f32 = tensor_as_vec_t
            .iter()
            .map(|i| i.iter().map(|q| q.to_f32().unwrap() ).collect())
            .collect();
        Self {
            name: name.into(),
            vec: tensor_as_vec_f32,
        }
    }
   
}
pub fn matrix_to_vec<T:candle_core::WithDType>(tensor: &Tensor) -> Vec<Vec<T>> {
    tensor.to_vec2::<T>().unwrap()
}