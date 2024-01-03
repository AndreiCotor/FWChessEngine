use crate::constants::{BOARD_SIZE, NUM_SQUARES};
use crate::exceptions::PieceError;

pub fn is_pawn_move_valid(from: u64, to: u64, orientation: bool) -> Result<(), PieceError> {

    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (from_rank, rank_diff, file_diff) = result.unwrap();

    if orientation {
        is_pawn_move_valid_bottom_to_top(from_rank, rank_diff, file_diff)
    } else {
        is_pawn_move_valid_top_to_bottom(from_rank, rank_diff, file_diff)
    }
}

fn basic_position_check(from: u64, to: u64) -> Result<(u64, u64, u64), PieceError> {
    if (from == to) || (from < 0) || (from >= NUM_SQUARES) || (to < 0) || (to >= NUM_SQUARES) {
        return Err(PieceError::InvalidMove);
    }

    let from_rank = from / 8;
    let to_rank = to / 8;

    let from_file = from % 8;
    let to_file = to % 8;

    if from_rank < 0 || from_rank >= BOARD_SIZE
        || to_rank < 0 || to_rank >= BOARD_SIZE {
        return Err(PieceError::InvalidMove);
    }

    let rank_diff = to_rank - from_rank;
    let file_diff = to_file - from_file;

    Ok((from_rank, rank_diff, file_diff))
}

fn is_pawn_move_valid_bottom_to_top(from_rank: u64, rank_diff: u64, file_diff: u64) -> Result<(), PieceError> {
    if from_rank == 1 && rank_diff == 2 && file_diff == 0 {
        return Ok(());
    }

    if rank_diff == 1 && file_diff == 0 {
        return Ok(());
    }

    if rank_diff == 1 && file_diff == 1 {
        return Ok(());
    }

    if rank_diff == 1 && file_diff == -1 {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

fn is_pawn_move_valid_top_to_bottom(from_rank: u64, rank_diff: u64, file_diff: u64) -> Result<(), PieceError> {
    if from_rank == 6 && rank_diff == -2 && file_diff == 0 {
        return Ok(());
    }

    if rank_diff == -1 && file_diff == 0 {
        return Ok(());
    }

    if rank_diff == -1 && file_diff == 1 {
        return Ok(());
    }

    if rank_diff == -1 && file_diff == -1 {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}