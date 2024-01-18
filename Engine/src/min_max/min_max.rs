use mpi::traits::*;
use std::cmp::{max, min};
use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::evaluator::evaluate;
use crate::min_max::next_move_generator::generate_next_moves;
use mpi::topology::SystemCommunicator;
const DEPTH: usize = 4;

fn min_max_with_alpha_beta_pruning(
    state: &Chessboard,
    depth: usize,
    mut alpha: i64,
    mut beta: i64,
    player_color: PlayerColor,
    rank: i32,
    size: i32,
    world: mpi::topology::SystemCommunicator,
) -> i64 {
    if depth == 0 || state.is_finished() {
        return evaluate(&state, player_color);
    }

    let possible_moves = generate_next_moves(&state, player_color);

    match player_color {
        PlayerColor::White => {
            let mut value = i64::MIN;
            let mut local_alpha = alpha;

            let chunk_size = (possible_moves.len() as f64 / size as f64).ceil() as usize;
            let start_idx = rank as usize * chunk_size;
            let end_idx = min((rank + 1) as usize * chunk_size, possible_moves.len());

            for next_move in possible_moves[start_idx..end_idx].iter() {
                let mut next_state = state.clone();
                let from = Chessboard::convert_index_to_square(next_move.0);
                let to = Chessboard::convert_index_to_square(next_move.1);
                next_state
                    .perform_move(&from, &to, PlayerColor::White)
                    .unwrap();

                value = max(
                    value,
                    min_max_with_alpha_beta_pruning(
                        &next_state,
                        depth - 1,
                        local_alpha,
                        beta,
                        PlayerColor::Black,
                        rank,
                        size,
                        world.clone(),
                    ),
                );

                if value > beta {
                    break;
                }

                local_alpha = max(local_alpha, value);
            }

            world.all_reduce_into(&local_alpha, &mut alpha, &mpi::collective::SystemOperation::max());

            value
        }
        PlayerColor::Black => {
            let mut value = i64::MAX;
            let mut local_beta = beta;

            let chunk_size = (possible_moves.len() as f64 / size as f64).ceil() as usize;
            let start_idx = rank as usize * chunk_size;
            let end_idx = min((rank + 1) as usize * chunk_size, possible_moves.len());

            for next_move in possible_moves[start_idx..end_idx].iter() {
                let mut next_state = state.clone();
                let from = Chessboard::convert_index_to_square(next_move.0);
                let to = Chessboard::convert_index_to_square(next_move.1);
                next_state
                    .perform_move(&from, &to, PlayerColor::Black)
                    .unwrap();

                value = min(
                    value,
                    min_max_with_alpha_beta_pruning(
                        &next_state,
                        depth - 1,
                        alpha,
                        local_beta,
                        PlayerColor::White,
                        rank,
                        size,
                        world.clone(),
                    ),
                );

                if value < alpha {
                    break;
                }

                local_beta = min(local_beta, value);
            }

            world.all_reduce_into(&local_beta, &mut beta, &mpi::collective::SystemOperation::min());

            value
        }
    }
}

pub fn get_best_move(state: &Chessboard, world: &SystemCommunicator, rank: i32, size: i32) -> (u64, u64) {

    let mut result = (0, 0);
    let possible_moves = generate_next_moves(&state, PlayerColor::White);
    let mut value = i64::MIN;

    let chunk_size = (possible_moves.len() as f64 / size as f64).ceil() as usize;
    let start_idx = rank as usize * chunk_size;
    let end_idx = min((rank + 1) as usize * chunk_size, possible_moves.len());
    let local_moves = &possible_moves[start_idx..end_idx];

    for next_move in local_moves.iter() {
        let mut next_state = state.clone();
        let from = Chessboard::convert_index_to_square(next_move.0);
        let to = Chessboard::convert_index_to_square(next_move.1);
        next_state
            .perform_move(&from, &to, PlayerColor::White)
            .unwrap();

        let min_max_value = min_max_with_alpha_beta_pruning(
            &next_state,
            DEPTH - 1,
            i64::MIN,
            i64::MAX,
            PlayerColor::Black,
            rank,
            size,
            world.clone(),
        );

        if min_max_value > value {
            value = min_max_value;
            result = *next_move;
        }
    }

    result
}

