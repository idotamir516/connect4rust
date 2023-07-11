use crate::Player::{Player1, Player2};
use rand::Rng;
use std::collections::HashSet;

const EMPTY_VALUE: u8 = 0;
const PLAYER_1_VALUE: u8 = 1;
const PLAYER_2_VALUE: u8 = 2;

#[derive(Clone, Copy, Debug)]
enum Player {
    Player1,
    Player2,
}

impl Player {
    fn value(&self) -> u8 {
        match *self {
            Player1 => PLAYER_1_VALUE,
            Player2 => PLAYER_2_VALUE,
        }
    }

    fn switch(player: Player) -> Player {
        match player {
            Player1 => Player2,
            Player2 => Player1,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Move {
    row: u8,
    column: u8,
}

const COLUMNS: usize = 7;
const ROWS: usize = 6;

struct Game {
    grid: [[u8; ROWS]; COLUMNS],
    winner: Option<Player>,
}

impl Game {
    fn new() -> Self {
        Game {
            grid: [[0u8; ROWS]; COLUMNS],
            winner: None,
        }
    }

    fn possible_moves(&self) -> HashSet<Move> {
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

    fn doMove(&self, m: Move, player: Player) -> Game {
        let mut new_winner = self.winner.clone();
        let mut new_grid = self.grid.clone();
        new_grid[m.column as usize][m.row as usize] = player.value();

        if new_winner.is_some() {
            return Game {
                grid: new_grid,
                winner: new_winner,
            };
        }

        // horizontal
        // horizontal right
        let mut side_index = (m.column + 1) as i8;
        let mut to_the_side = 0;
        while side_index < new_grid.len() as i8
            && new_grid[side_index as usize][m.row as usize] == player.value()
        {
            to_the_side += 1;
            side_index += 1;
        }

        // horizontal left
        side_index = m.column as i8 - 1;
        while side_index >= 0 && new_grid[side_index as usize][m.row as usize] == player.value() {
            to_the_side += 1;
            side_index -= 1;
        }

        if to_the_side > 2 {
            new_winner = Some(player);
            return Game {
                grid: new_grid,
                winner: new_winner,
            };
        }

        // vertical
        // vertical above
        let mut vert_index: i8 = (m.row + 1) as i8;
        let mut above_or_below = 0;
        while vert_index < new_grid[0].len() as i8
            && new_grid[m.column as usize][vert_index as usize] == player.value()
        {
            above_or_below += 1;
            vert_index += 1;
        }

        // vertical below
        vert_index = m.row as i8 - 1;
        while vert_index >= 0 && new_grid[m.column as usize][vert_index as usize] == player.value()
        {
            above_or_below += 1;
            vert_index -= 1;
        }

        if above_or_below > 2 {
            new_winner = Some(player);
            return Game {
                grid: new_grid,
                winner: new_winner,
            };
        }

        // _|
        // horizontal right
        let mut diagonal_index: i8 = 1;
        let mut diagonal = 0;
        while m.column as i8 + diagonal_index < COLUMNS as i8
            && m.row as i8 + diagonal_index < ROWS as i8
            && new_grid[(m.column as i8 + diagonal_index) as usize]
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
            && new_grid[(m.column as i8 + diagonal_index) as usize]
                [(m.row as i8 + diagonal_index) as usize]
                == player.value()
        {
            diagonal += 1;
            diagonal_index -= 1;
        }

        if diagonal > 2 {
            new_winner = Some(player);
            return Game {
                grid: new_grid,
                winner: new_winner,
            };
        }

        // -|
        // horizontal right
        diagonal_index = 1;
        diagonal = 0;
        while m.column as i8 + diagonal_index < COLUMNS as i8
            && m.row as i8 - diagonal_index >= 0 as i8
            && new_grid[(m.column as i8 + diagonal_index) as usize]
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
            && new_grid[(m.column as i8 - diagonal_index) as usize]
                [(m.row as i8 + diagonal_index) as usize]
                == player.value()
        {
            diagonal += 1;
            diagonal_index += 1;
        }

        if diagonal > 2 {
            new_winner = Some(player);
            return Game {
                grid: new_grid,
                winner: new_winner,
            };
        }

        return Game {
            grid: new_grid,
            winner: new_winner,
        };
    }

    fn do_move(&mut self, m: Move, player: Player) {
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

    fn position_emoji(value: u8) -> char {
        match value {
            PLAYER_1_VALUE => '🔴',
            PLAYER_2_VALUE => '🔵',
            EMPTY_VALUE => '⚪',
            _ => panic!("Undefined emoji value"),
        }
    }

    fn emoji_view(&self) -> String {
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

trait Strategy {
    fn best_move(&self, player: Player, game: &Game) -> Move;
}

struct RandomStrategy {}
impl Strategy for RandomStrategy {
    fn best_move(&self, player: Player, game: &Game) -> Move {
        let possible_moves = game.possible_moves();
        let move_index = rand::thread_rng().gen_range(0..possible_moves.len());
        return *possible_moves.iter().skip(move_index).nth(0).unwrap();
    }
}

fn main() {
    // let mut game = Game::new();
    // let mut player = Player1;
    // for _ in 0..25 {
    //     let best_move = RandomStrategy{}.best_move(player, &game);
    //     game = game.doMove(best_move, player);
    //     println!("{:?}", best_move);
    //     println!("{}", game.emoji_view());
    //     println!("{:?}", game.winner);
    //     player = Player::switch(player);
    // }

    use std::time::Instant;
    let now = Instant::now();
    let mut total_turns: i64 = 0;
    // hashmap<Moves, (Score, Count)> map
    // hashmap<GameHash, (Score, Count)> map
    for _ in 0..1_00_000 {
        let mut game = Game::new();
        let mut player = Player1;
        let mut turns = 0;
        // list player1Moves, player2Moves
        // list player1States, player2States,
        let strategy = RandomStrategy {};
        while game.winner.is_none() && turns < ROWS * COLUMNS - 1 {
            let best_move = strategy.best_move(player, &game);
            // add move to right player
            // README: so 2 options here, copy the game or don't copy the game every time a move is
            // done. What I have now is the copy method. If we want to change to the mutate method
            // replace line 396 with the following:
            game.do_move(best_move, player);
            // game = game.doMove(best_move, player);
            // add new state to both players
            player = Player::switch(player);
            turns += 1;
        }

        // if player1 Won add 1 point to all player1Moves and subtract 1 point to all player2Moves
        // else vice versa

        total_turns += turns as i64;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("total turns: {}", total_turns);
}
// future notes
// store game state hash -> percent won need to be reversible
// store ti;erce t person with token in position won game -> hash can be reversible, always points to your team
// !!!!!!!! Csondier using i:128 where each bit represents a position and 0/1 represents your position and 0/1 represents your opponent on the 2nd half of bits
// scoring mechanism -> (state won / (number of times in state + 3)) * 2 * (token in position won)^2

// test that winner is correctly found
// game.grid[0][0] = PLAYER_1_VALUE;
// game.grid[0][1] = PLAYER_1_VALUE;
// game.grid[0][2] = PLAYER_1_VALUE;
// println!("{:?}", game.doMove(Move {column: 0, row: 3}, Player::Player1).winner);
