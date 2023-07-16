use std::collections::HashSet;

use crate::player::Player;
use crate::player::PLAYER_1_VALUE;
use crate::player::PLAYER_2_VALUE;
use crate::r#move::Move;

pub const EMPTY_VALUE: u8 = 0;

pub const COLUMNS: usize = 7;
pub const ROWS: usize = 6;
pub const BOARD_SIZE: usize = COLUMNS * ROWS;

pub struct Game {
    pub grid: [[u8; ROWS]; COLUMNS],
    pub winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: [[0u8; ROWS]; COLUMNS],
            winner: None,
        }
    }

    pub fn possible_moves(&self) -> HashSet<Move> {
        let mut possible_moves = HashSet::new();
        for i in 0..COLUMNS {
            let mut max_index = 0;
            for j in 0..ROWS {
                if self.grid[i][j] != 0 {
                    max_index += 1;
                } else {
                    break;
                }
            }
            if max_index != ROWS {
                possible_moves.insert(Move {
                    column: i as u8,
                    row: max_index as u8,
                });
            }
        }

        return possible_moves;
    }

    pub fn do_move(&mut self, m: Move, player: Player) {
        self.grid[m.column as usize][m.row as usize] = player.value();

        if self.winner.is_some() {
            return;
        }

        // horizontal
        // horizontal right
        let mut side_index = (m.column + 1) as i8;
        let mut to_the_side = 0;
        while side_index < self.grid.len() as i8
            && self.grid[side_index as usize][m.row as usize] == player.value()
        {
            to_the_side += 1;
            side_index += 1;
        }

        // horizontal left
        side_index = m.column as i8 - 1;
        while side_index >= 0 && self.grid[side_index as usize][m.row as usize] == player.value() {
            to_the_side += 1;
            side_index -= 1;
        }

        if to_the_side > 2 {
            self.winner = Some(player);
            return;
        }

        // vertical
        // vertical above
        let mut vert_index: i8 = (m.row + 1) as i8;
        let mut above_or_below = 0;
        while vert_index < self.grid[0].len() as i8
            && self.grid[m.column as usize][vert_index as usize] == player.value()
        {
            above_or_below += 1;
            vert_index += 1;
        }

        // vertical below
        vert_index = m.row as i8 - 1;
        while vert_index >= 0 && self.grid[m.column as usize][vert_index as usize] == player.value()
        {
            above_or_below += 1;
            vert_index -= 1;
        }

        if above_or_below > 2 {
            self.winner = Some(player);
            return;
        }

        // _|
        // horizontal right
        let mut diagonal_index: i8 = 1;
        let mut diagonal = 0;
        while m.column as i8 + diagonal_index < COLUMNS as i8
            && m.row as i8 + diagonal_index < ROWS as i8
            && self.grid[(m.column as i8 + diagonal_index) as usize]
            [(m.row as i8 + diagonal_index) as usize]
            == player.value()
        {
            diagonal += 1;
            diagonal_index += 1;
        }

        // horizontal left
        diagonal_index = -1;
        while m.column as i8 + diagonal_index >= 0
            && m.row as i8 + diagonal_index >= 0
            && self.grid[(m.column as i8 + diagonal_index) as usize]
            [(m.row as i8 + diagonal_index) as usize]
            == player.value()
        {
            diagonal += 1;
            diagonal_index -= 1;
        }

        if diagonal > 2 {
            self.winner = Some(player);
            return;
        }

        // -|
        // horizontal right
        diagonal_index = 1;
        diagonal = 0;
        while m.column as i8 + diagonal_index < COLUMNS as i8
            && m.row as i8 - diagonal_index >= 0 as i8
            && self.grid[(m.column as i8 + diagonal_index) as usize]
            [(m.row as i8 - diagonal_index) as usize]
            == player.value()
        {
            diagonal += 1;
            diagonal_index += 1;
        }

        // horizontal left
        diagonal_index = 1;
        while m.column as i8 - diagonal_index >= 0
            && m.row as i8 + diagonal_index < ROWS as i8
            && self.grid[(m.column as i8 - diagonal_index) as usize]
            [(m.row as i8 + diagonal_index) as usize]
            == player.value()
        {
            diagonal += 1;
            diagonal_index += 1;
        }

        if diagonal > 2 {
            self.winner = Some(player);
            return;
        }
    }

    pub fn calculate_hash(&self, player: Player) -> i128 {
        let mut hash: i128 = 0;
        let mut index = 0;
        for column in self.grid {
            for val in column {
                let val_index = match val {
                    EMPTY_VALUE => 0,
                    _ if val == player.value() => 1 << index,
                    _ => 1 << (index + BOARD_SIZE)
                };
                hash += val_index;
                index += 1;
            }
        }

        return hash;
    }

    fn position_emoji(value: u8) -> char {
        match value {
            PLAYER_1_VALUE => '🔴',
            PLAYER_2_VALUE => '🔵',
            EMPTY_VALUE => '⚪',
            _ => panic!("Undefined emoji value"),
        }
    }

    pub fn emoji_view(&self) -> String {
        let mut view = String::with_capacity(ROWS * COLUMNS + ROWS);
        for j in (0..ROWS).rev() {
            for i in 0..COLUMNS {
                view.push(Game::position_emoji(self.grid[i][j]));
            }
            view.push('\n');
        }
        return view;
    }
}

#[cfg(test)]
mod test_game {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hash() {
        let mut game = Game::new();
        assert_eq!(game.calculate_hash(Player1), 0);
        game.do_move(Move { row: 0, column: 0 }, Player1);
        assert_eq!(game.calculate_hash(Player1), 1);
        assert_eq!(game.calculate_hash(Player2), 1 << BOARD_SIZE);
        game.do_move(Move { row: 1, column: 0 }, Player2);
        assert_eq!(game.calculate_hash(Player1), 1 + (1 << (BOARD_SIZE + 1)));
        assert_eq!(game.calculate_hash(Player2), 2 + (1 << BOARD_SIZE));
    }
}