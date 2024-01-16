use std::cmp::{max, min};
use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::evaluator::evaluate;
use crate::min_max::next_move_generator::generate_next_moves;

const DEPTH: usize = 2;

fn min_max_with_alpha_beta_pruning(
    state: &Chessboard,
    depth: usize,
    mut alpha: i64,
    mut beta: i64,
    player_color: PlayerColor
) -> i64 {

    if depth == 0 || state.is_finished() {
        return evaluate(&state, player_color);
    }

    let possible_moves = generate_next_moves(&state, player_color);

    match player_color {
        PlayerColor::White => {
            let mut value = i64::MIN;
            for next_move in possible_moves {
                let mut next_state = state.clone();

                let from = Chessboard::convert_index_to_square(next_move.0);
                let to = Chessboard::convert_index_to_square(next_move.1);
                next_state.perform_move(&from, &to, PlayerColor::White).unwrap();

                value = max(
                    value,
                    min_max_with_alpha_beta_pruning(&next_state, depth - 1, alpha, beta, PlayerColor::Black)
                );

                if value > beta {
                    break;
                }

                alpha = max(alpha, value)
            }
            value
        },
        PlayerColor::Black => {
            let mut value = i64::MAX;
            for next_move in possible_moves {
                let mut next_state = state.clone();

                let from = Chessboard::convert_index_to_square(next_move.0);
                let to = Chessboard::convert_index_to_square(next_move.1);
                next_state.perform_move(&from, &to, PlayerColor::Black).unwrap();

                value = min(
                    value,
                    min_max_with_alpha_beta_pruning(&next_state, depth - 1, alpha, beta, PlayerColor::White)
                );

                if value < alpha {
                    break;
                }

                beta = min(beta, value)
            }
            value
        }
    }
}

pub fn get_best_move(state: &Chessboard) -> (u64, u64) {
    let mut result = (0, 0);
    let possible_moves = generate_next_moves(&state, PlayerColor::White);
    let mut value = i64::MIN;
    for next_move in possible_moves {
        let mut next_state = state.clone();

        let from = Chessboard::convert_index_to_square(next_move.0);
        let to = Chessboard::convert_index_to_square(next_move.1);
        next_state.perform_move(&from, &to, PlayerColor::White).unwrap();

        let min_max_value = min_max_with_alpha_beta_pruning(&next_state, DEPTH, i64::MIN, i64::MAX, PlayerColor::Black);

        if min_max_value > value {
            value = min_max_value;
            result = next_move;
        }
    }

    result
}