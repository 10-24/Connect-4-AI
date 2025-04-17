use candle_core::{Device, Tensor};
use candle_nn::{Linear, Module, VarBuilder};

use crate::connect_four::game_board::GameBoard;



#[derive(Debug)]
pub struct ConnectFourNN {
    pub layers: [Linear; 3],
    device: Device,
}
impl ConnectFourNN {
    pub fn new(vb: VarBuilder, device: Device) -> Result<Self, candle_core::Error> {
        // let layers: [Linear; 3] = [
        //     candle_nn::linear(GameBoard::TOTAL_SPACES, 128, vb.pp("layer_1"))?, // 42 inputs (board), 128 hidden
        //     candle_nn::linear(128, 64, vb.pp("layer_2"))?, // 64 hidden
        //     candle_nn::linear(64, GameBoard::COLS, vb.pp("layer_3"))?,   // 7 outputs (columns)
        // ];
        let layers: [Linear; 3] = [
            candle_nn::linear(GameBoard::TOTAL_SPACES, 42, vb.pp("layer_1"))?, // 42 inputs (board), 128 hidden
            candle_nn::linear(42, 20, vb.pp("layer_2"))?, // 64 hidden
            candle_nn::linear(20, GameBoard::COLS, vb.pp("layer_3"))?,   // 7 outputs (columns)
        ];

        Ok(Self { layers, device })
    }
}
impl Module for ConnectFourNN {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let xs = self.layers[0].forward(&xs)?.relu()?;
        let xs = self.layers[1].forward(&xs)?.relu()?;
        let xs = self.layers[2].forward(&xs)?;
        Ok(xs)
    }
}
