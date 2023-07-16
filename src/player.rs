use crate::player::Player::{Player1, Player2};

pub const PLAYER_1_VALUE: u8 = 1;
pub const PLAYER_2_VALUE: u8 = 2;

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn value(&self) -> u8 {
        match *self {
            Player1 => PLAYER_1_VALUE,
            Player2 => PLAYER_2_VALUE,
        }
    }

    pub fn switch(player: Player) -> Player {
        match player {
            Player1 => Player2,
            Player2 => Player1,
        }
    }
}