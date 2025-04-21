// use std::{
//     fs::{self, File, OpenOptions},
//     io::Write,
//     path::Path,
//     str::FromStr,
// };

// use candle_core::Tensor;
// use chrono::Local;
// use derive_new::new;
// use serde::{Serialize, Serializer};

// const LOG_FOLDER: &str = "logs";

// pub struct Logger {}
// impl Logger {
//     pub fn log(log_name: &str, new_log: &[String]) {
//         let mut file = Self::get_or_create_file(log_name);
//         let csv = new_log.join(",");
//         writeln!(file, "{csv}");
//     }

//     pub fn log_tensor_2d_f32(log_name: &str, prefix: &[String], tensor: &Tensor) {
//         let tensor_as_vec = tensor.to_vec2::<f32>().unwrap();
//         for (i, tensor_row) in tensor_as_vec.iter().enumerate() {
//             let mut csv_row: Vec<String> = prefix.to_vec();
//             csv_row.push(i.to_string());

//             csv_row.extend(tensor_row.iter().map(|v| v.to_string()));
//             Self::log(log_name, csv_row.as_slice());
//         }
//     }
//     pub fn log_tensor_2d_u32(log_name: &str, prefix: &[String], tensor: &Tensor) {
//         let tensor_as_vec = tensor.to_vec2::<u32>().unwrap();
//         for (i, tensor_row) in tensor_as_vec.iter().enumerate() {
//             let mut csv_row: Vec<String> = prefix.to_vec();
//             csv_row.push(format!("[{i}]"));

//             csv_row.extend(tensor_row.iter().map(|v| v.to_string()));
//             Self::log(log_name, csv_row.as_slice());
//         }
//     }

//     pub fn log_tensor_1d_f32(log_name: &str, prefix: &[String], tensor: &Tensor) {
//         let mut row: Vec<String> = prefix.to_vec();
//         let tensor_as_vec = tensor.to_vec1::<f32>().unwrap();
//         let tensor_as_iter = tensor_as_vec.iter().map(|i| i.to_string());
//         row.extend(tensor_as_iter);
//         Self::log(log_name, row.as_slice());
//     }

//     fn get_or_create_file(log_name: &str) -> File {
//         let file_path = format!("{LOG_FOLDER}/{log_name}.csv");
//         // Create the logs directory if it doesn't exist
//         fs::create_dir_all(LOG_FOLDER);
//         // Open file in write mode, append if it exists, create if it doesn't
//         OpenOptions::new()
//             .write(true)
//             .append(true)
//             .create(true)
//             .open(file_path)
//             .unwrap()
//     }
//     fn get_timestamp() -> String {
//         Local::now().format("%m-%d_%H-%M").to_string()
//     }
// }
