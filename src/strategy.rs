use crate::game::Game;
use crate::player::Player;
use crate::r#move::Move;

pub trait Strategy {
    fn best_move(&self, player: Player, game: &Game) -> Move;
}