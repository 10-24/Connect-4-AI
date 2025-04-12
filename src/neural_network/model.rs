use candle_core::{Device, Tensor};
use candle_nn::{Linear, Module, VarBuilder};

use crate::connect_four::connect_four::ConnectFour;


#[derive(Debug)]
pub struct ConnectFourNN {
    layers: [Linear; 3],
    device: Device,
}
impl ConnectFourNN {
    pub fn new(vb: VarBuilder, device: Device) -> Result<Self, candle_core::Error> {
        let layers: [Linear; 3] = [
            candle_nn::linear(ConnectFour::TOTAL_SPACES, 128, vb.pp("layer1"))?, // 42 inputs (board), 128 hidden
            candle_nn::linear(128, 64, vb.pp("layer2"))?, // 64 hidden
            candle_nn::linear(64, ConnectFour::COLS, vb.pp("layer3"))?,   // 7 outputs (columns)
        ];

        Ok(Self { layers, device })
    }
}
impl Module for ConnectFourNN {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let mut xs: Tensor = self.layers[0].forward(xs)?.relu()?;

        for layer in self.layers.iter().skip(1) {
            xs = layer.forward(&xs)?.relu()?;
        }

        Ok(xs)
    }
}
