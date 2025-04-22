use burn::{prelude::Backend, tensor::{Bool, Int, Tensor}};
use derive_new::new;

use crate::training::optimize_model::tensor_ops::create_mask;

#[derive(new)]
pub struct TargetQValBuilder<B: Backend> {
    selected_cols:Tensor<B,1,Int>,
    state_values:Tensor<B,1>,
}

impl<B: Backend> TargetQValBuilder<B> {
    pub fn build(self,predicted_q_vals:Tensor<B,2>,device: &B::Device) -> Tensor<B, 2> {
        let target_q_vals_shape = predicted_q_vals.shape();

        let selected_col_mask:Tensor<B,2,Bool> = create_mask(self.selected_cols.clone(), target_q_vals_shape.clone(),device);
        let values_2d_full = self.state_values
            .unsqueeze_dim::<2>(1)
            .expand(target_q_vals_shape.clone());
    
        predicted_q_vals.mask_where(selected_col_mask, values_2d_full)
    }
}