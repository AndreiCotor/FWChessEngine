use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use async_recursion::async_recursion;
use tokio::task;
use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::evaluator::evaluate;
use crate::min_max::next_move_generator::generate_next_moves;

const DEPTH: usize = 4;

#[async_recursion]
async fn min_max_with_alpha_beta_pruning(
    state: &Chessboard,
    depth: usize,
    alpha: i64,
    beta: i64,
    player_color: PlayerColor
) -> i64 {

    if depth == 0 || state.is_finished() {
        return evaluate(&state, player_color);
    }

    let possible_moves = generate_next_moves(&state, player_color);

    match player_color {
        PlayerColor::White => {
            let value_atomic = Arc::new(AtomicI64::new(i64::MIN));
            let alpha_atomic = Arc::new(AtomicI64::new(alpha));
            let mut joins = vec![];

            for next_move in possible_moves {
                let mut next_state = state.clone();
                let value_clone = value_atomic.clone();
                let alpha_clone = alpha_atomic.clone();

                joins.push(task::spawn(async move {
                    let from = Chessboard::convert_index_to_square(next_move.0);
                    let to = Chessboard::convert_index_to_square(next_move.1);
                    next_state.perform_move(&from, &to, PlayerColor::White).unwrap();

                    if value_clone.load(Ordering::Relaxed) < beta {
                        let result = min_max_with_alpha_beta_pruning(
                            &next_state,
                            depth - 1,
                            alpha_clone.load(Ordering::Relaxed),
                            beta,
                            PlayerColor::Black
                        ).await;

                        value_clone.fetch_max(result, Ordering::Relaxed);

                        alpha_clone.fetch_max(result, Ordering::Relaxed);
                    }
                }));
            }
            for join in joins {
                join.await.unwrap();
            }

            value_atomic.load(Ordering::Relaxed)
        },
        PlayerColor::Black => {
            let value_atomic = Arc::new(AtomicI64::new(i64::MAX));
            let beta_atomic = Arc::new(AtomicI64::new(beta));
            let mut joins = vec![];

            for next_move in possible_moves {
                let mut next_state = state.clone();
                let value_clone = value_atomic.clone();
                let beta_clone = beta_atomic.clone();

                joins.push(task::spawn(async move {
                    let from = Chessboard::convert_index_to_square(next_move.0);
                    let to = Chessboard::convert_index_to_square(next_move.1);
                    next_state.perform_move(&from, &to, PlayerColor::Black).unwrap();

                    if value_clone.load(Ordering::Relaxed) > alpha {
                        let result = min_max_with_alpha_beta_pruning(
                            &next_state,
                            depth - 1,
                            alpha,
                            beta_clone.load(Ordering::Relaxed),
                            PlayerColor::White
                        ).await;

                        value_clone.fetch_min(result, Ordering::Relaxed);

                        beta_clone.fetch_min(result, Ordering::Relaxed);
                    }
                }));
            }
            for join in joins {
                join.await.unwrap();
            }

            value_atomic.load(Ordering::Relaxed)
        }
    }
}

pub async fn get_best_move(state: &Chessboard) -> (u64, u64) {
    let mut result = (0, 0);
    let possible_moves = generate_next_moves(&state, PlayerColor::White);
    let mut value = i64::MIN;
    for next_move in possible_moves {
        let mut next_state = state.clone();

        let from = Chessboard::convert_index_to_square(next_move.0);
        let to = Chessboard::convert_index_to_square(next_move.1);
        next_state.perform_move(&from, &to, PlayerColor::White).unwrap();

        let min_max_value = min_max_with_alpha_beta_pruning(&next_state, DEPTH, i64::MIN, i64::MAX, PlayerColor::Black).await;

        if min_max_value > value {
            value = min_max_value;
            result = next_move;
        }
    }

    result
}