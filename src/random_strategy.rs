use rand::Rng;
use crate::r#move::Move;
use crate::game::Game;
use crate::player::Player;
use crate::strategy::Strategy;

pub struct RandomStrategy {}

impl Strategy for RandomStrategy {
    fn best_move(&self, _player: Player, game: &Game) -> Move {
        let possible_moves = game.possible_moves();
        let move_index = rand::thread_rng().gen_range(0..possible_moves.len());
        return *possible_moves.iter().skip(move_index).nth(0).unwrap();
    }
}
