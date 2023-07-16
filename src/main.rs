use crate::Player::{Player1, Player2};
use rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};

const EMPTY_VALUE: u8 = 0;
const PLAYER_1_VALUE: u8 = 1;
const PLAYER_2_VALUE: u8 = 2;

const COLUMNS: usize = 7;
const ROWS: usize = 6;
const BOARD_SIZE: usize = COLUMNS * ROWS;

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

#[derive(Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Move {
    row: u8,
    column: u8,
}

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

    fn calculate_hash(&self, player: Player) -> i128 {
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

#[derive(Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Score {
    count: u32,
    score: i32,
}

impl Score {
    fn new() -> Self {
        Score {
            count: 0,
            score: 0,
        }
    }

    fn plus_one(&mut self) -> &mut Score {
        self.score += 1;
        self.count += 1;
        return self;
    }

    fn minus_one(&mut self) -> &mut Score {
        self.score -= 1;
        self.count += 1;
        return self;
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
    let mut move_scores: HashMap<Move, Score> = HashMap::new();
    let mut state_scores: HashMap<i128, Score> = HashMap::new();
    for _ in 0..1_000 {
        let mut game = Game::new();
        let mut player = Player1;
        let mut turns = 0;
        let mut player_1_moves = Vec::new();
        let mut player_2_moves = Vec::new();
        let mut player_1_states = Vec::new();
        let mut player_2_states = Vec::new();
        let strategy = RandomStrategy {};
        while game.winner.is_none() && turns < BOARD_SIZE - 1 {
            let best_move = strategy.best_move(player, &game);
            // add move to right player
            match player {
                Player1 => player_1_moves.push(best_move),
                Player2 => player_2_moves.push(best_move),
            }

            game.do_move(best_move, player);
            // add new state to both players
            player_1_states.push(game.calculate_hash(Player1));
            player_2_states.push(game.calculate_hash(Player2));
            player = Player::switch(player);
            turns += 1;
        }

        // if player1 Won add 1 point to all player1Moves and subtract 1 point to all player2Moves
        // else vice versa
        match game.winner {
            Some(Player1) => {
                // let mut emptyScore = &mut Score::new();
                for player_1_move in player_1_moves {
                    *move_scores.entry(player_1_move).or_insert_with(Score::new).plus_one();
                    // let mut score = move_scores.entry(player_1_move).or_insert_with(|| *emptyScore);
                    // let mut score = move_scores.get_mut(&player_1_move).unwrap_or(emptyScore);
                //     let option = move_scores.get_mut(&player_1_move);
                //     let mut score = match option {
                //         Some(score) => score,
                //         None => emptyScore,
                //     };
                // let mut score = move_scores.get_mut(&player_1_move).unwrap_or(emptyScore).plus_one();
                    // match move_scores.get_mut(&player_1_move) {
                    //     Some(score) => {
                    //         score.plus_one();
                    //         move_scores.insert(player_1_move, *score);
                    //     },
                    //     None => {
                    //         let mut score = Score::new();
                    //         score.plus_one();
                    //         move_scores.insert(player_1_move, score);
                    //     }
                    // }
                    // let mut score = move_scores.get(&player_1_move);
                    // let mut score = score.unwrap_or(&Score::new());
                    // let mut score = score.plus_one();
                    // move_scores.insert(player_1_move, *score);
                }

                for player_2_move in player_2_moves {
                    *move_scores.entry(player_2_move).or_insert_with(Score::new).minus_one();
                    // let score = move_scores.get(&player_2_move).unwrap_or(&Score::new()).minus_one();
                    // move_scores.insert(player_2_move, *score);
                }

                for player_1_state in player_1_states {
                    *state_scores.entry(player_1_state).or_insert_with(Score::new).plus_one();
                    // let score = state_scores.get(&player_1_state).unwrap_or(&Score::new()).plus_one();
                    // state_scores.insert(player_1_state, *score);
                }

                for player_2_state in player_2_states {
                    *state_scores.entry(player_2_state).or_insert_with(Score::new).minus_one();
                    // let score = state_scores.get(&player_2_state).unwrap_or(&Score::new()).minus_one();
                    // state_scores.insert(player_2_state, *score);
                }
            }
            Some(Player2) => {
                for player_1_move in player_1_moves {
                    *move_scores.entry(player_1_move).or_insert_with(Score::new).minus_one();
                    // let score = move_scores.get(&player_1_move).unwrap_or(&Score::new()).minus_one();
                    // move_scores.insert(player_1_move, *score);
                }

                for player_2_move in player_2_moves {
                    *move_scores.entry(player_2_move).or_insert_with(Score::new).plus_one();
                    // let score = move_scores.get(&player_2_move).unwrap_or(&Score::new()).plus_one();
                    // move_scores.insert(player_2_move, *score);
                }

                for player_1_state in player_1_states {
                    *state_scores.entry(player_1_state).or_insert_with(Score::new).minus_one();
                    // let score = state_scores.get(&player_1_state).unwrap_or(&Score::new()).minus_one();
                    // state_scores.insert(player_1_state, *score);
                }

                for player_2_state in player_2_states {
                    *state_scores.entry(player_2_state).or_insert_with(Score::new).plus_one();
                    // let score = state_scores.get(&player_2_state).unwrap_or(&Score::new()).plus_one();
                    // state_scores.insert(player_2_state, *score);
                }
            },
            None => {}
        }

        total_turns += turns as i64;
    }

    let move_scores_file = OpenOptions::new().write(true).create(true).open("move_scores.txt").unwrap();
    let state_scores_file = OpenOptions::new().write(true).create(true).open("state_scores.txt").unwrap();
    let mut move_score_string = HashMap::new();
    for (move_key, score) in move_scores {
        move_score_string.insert(serde_json::to_string(&move_key).unwrap(), score);
    }
    let mut state_score_string = HashMap::new();
    for (state, score) in state_scores {
        state_score_string.insert(state.to_string(), score);
    }
    serde_json::to_writer(move_scores_file, &move_score_string).unwrap();
    serde_json::to_writer(state_scores_file, &state_score_string).unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("total turns: {}", total_turns);

    // next steps:
    // linear steps:
    // 1. reimplement what we are writing to file. I forgot to add the count (DONE)
    // 2. run algorithm on large game set and in parallel write strategy that takes the data
    //    and uses it to decide which move is best
    // 3. Show that strategy from step 2 beats random strategy
    // 4. Train strategy from 3 against itself and calculate new map. In parallel write strategy
    //     that takes data but also uses min max
    // 5. Show that strategy from step 2 beats random strategy and strategy from step 3
    // 6. Add random moves to strategy to encourage exploration and get it to self learn again
    // parallel steps
    // 1. refactor file to be multi file project
}
// future notes
// store game state hash -> percent won need to be reversible
// store ti;erce t person with token in position won game -> hash can be reversible, always points to your team
// !!!!!!!! Csondier using i:128 where each bit represents a position and 0/1 represents your position and 0/1 represents your opponent on the 2nd half of bits
// scoring mechanism -> (state won / (number of times in state + 3)) * 2 + (token in position won)^2

// test that winner is correctly found
// game.grid[0][0] = PLAYER_1_VALUE;
// game.grid[0][1] = PLAYER_1_VALUE;
// game.grid[0][2] = PLAYER_1_VALUE;
// println!("{:?}", game.doMove(Move {column: 0, row: 3}, Player::Player1).winner);
