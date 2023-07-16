mod player;
mod r#move;
mod game;
mod strategy;
mod random_strategy;
mod score;

use std::collections::HashMap;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use crate::player::Player;
use crate::player::Player::{Player1, Player2};
use crate::r#move::Move;
use crate::game::Game;
use crate::game::BOARD_SIZE;
use crate::strategy::Strategy;

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
    use random_strategy::RandomStrategy;
    use score::Score;
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
                for player_1_move in player_1_moves {
                    move_scores.entry(player_1_move).or_insert_with(Score::new).plus_one();
                }

                for player_2_move in player_2_moves {
                    move_scores.entry(player_2_move).or_insert_with(Score::new).minus_one();
                }

                for player_1_state in player_1_states {
                    state_scores.entry(player_1_state).or_insert_with(Score::new).plus_one();
                }

                for player_2_state in player_2_states {
                    state_scores.entry(player_2_state).or_insert_with(Score::new).minus_one();
                }
            }
            Some(Player2) => {
                for player_1_move in player_1_moves {
                    move_scores.entry(player_1_move).or_insert_with(Score::new).minus_one();
                }

                for player_2_move in player_2_moves {
                    move_scores.entry(player_2_move).or_insert_with(Score::new).plus_one();
                }

                for player_1_state in player_1_states {
                    state_scores.entry(player_1_state).or_insert_with(Score::new).minus_one();
                }

                for player_2_state in player_2_states {
                    state_scores.entry(player_2_state).or_insert_with(Score::new).plus_one();
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
