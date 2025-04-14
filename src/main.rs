use player::Player;
use ui::spectate_game::{self, GameTurn};

mod connect_four;
mod neural_network;
mod player;
mod ui;

fn main() {
    let game_record = vec![
        GameTurn::new(Player::Blue, 1),
        GameTurn::new(Player::Red, 2),
    ];
    spectate_game::spectate_game(game_record);
    // train();
}
