use crate::constants::{BOARD_SIZE, NUM_SQUARES};
use crate::exceptions::PieceError;
use crate::player::Player;

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

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

fn is_pawn_move_valid_bottom_to_top(from_rank: u64, rank_diff: i64, file_diff: i64) -> Result<(), PieceError> {
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

fn is_pawn_move_valid_top_to_bottom(from_rank: u64, rank_diff: i64, file_diff: i64) -> Result<(), PieceError> {
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


pub fn is_knight_move_valid(from: u64, to: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if (rank_diff == 2 && file_diff == 1)
        || (rank_diff == 2 && file_diff == -1)
        || (rank_diff == -2 && file_diff == 1)
        || (rank_diff == -2 && file_diff == -1)
        || (rank_diff == 1 && file_diff == 2)
        || (rank_diff == 1 && file_diff == -2)
        || (rank_diff == -1 && file_diff == 2)
        || (rank_diff == -1 && file_diff == -2) {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

pub fn is_bishop_move_valid(from: u64, to: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff == file_diff || rank_diff == -file_diff {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

pub fn is_rook_move_valid(from: u64, to: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff == 0 || file_diff == 0 {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

pub fn is_queen_move_valid(from: u64, to: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff == 0 || file_diff == 0 || rank_diff == file_diff || rank_diff == -file_diff {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

pub fn is_king_move_valid(from: u64, to: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff.abs() <= 1 && file_diff.abs() <= 1 {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

fn basic_position_check(from: u64, to: u64) -> Result<(u64, i64, i64), PieceError> {
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

    let rank_diff: i64 = (to_rank - from_rank) as i64;
    let file_diff: i64 = (to_file - from_file) as i64;

    Ok((from_rank, rank_diff, file_diff))
}

pub fn check_pawn_move_blocked(from: u64, to: u64, orientation: bool, board: u64, white_board: u64, black_board: u64) -> bool {
    let (from_rank, rank_diff, file_diff) = basic_position_check(from, to).unwrap();

    if orientation {
        check_pawn_move_blocked_bottom_to_top(from_rank, rank_diff, file_diff, board, white_board, black_board, to)
    } else {
        check_pawn_move_blocked_top_to_bottom(from_rank, rank_diff, file_diff, board, white_board, black_board, to)
    }
}

fn check_pawn_move_blocked_bottom_to_top(from_rank: u64, rank_diff: i64, file_diff: i64, board: u64, white_board: u64, black_board: u64, to: u64) -> bool {
    //  account for en passant and promotion and capture

    if (white_board & (1 << to)) != 0 {
        return true;
    }

    if from_rank == 1 && rank_diff == 2 && file_diff == 0 {
        return (board & (1 << (from_rank + 1) * 8 + file_diff as u64)) != 0
            || (board & (1 << (from_rank + 2) * 8 + file_diff as u64)) != 0;
    }

    if rank_diff == 1 && file_diff == 0 {
        return (board & (1 << (from_rank + 1) * 8 + file_diff as u64)) != 0;
    }

    if rank_diff == 1 && file_diff == 1 {
        return (black_board & (1 << (from_rank + 1) * 8 + file_diff as u64)) != 0;
    }

    if rank_diff == 1 && file_diff == -1 {
        return (black_board & (1 << (from_rank + 1) * 8 + file_diff as u64)) != 0;
    }

    false
}

fn check_pawn_move_blocked_top_to_bottom(from_rank: u64, rank_diff: i64, file_diff: i64, board: u64, white_board: u64, black_board: u64, to: u64) -> bool {

    if (black_board & (1 << to)) != 0 {
        return true;
    }

    if from_rank == 6 && rank_diff == -2 && file_diff == 0 {
        return (board & (1 << (from_rank - 1) * 8 + file_diff as u64)) != 0
            || (board & (1 << (from_rank - 2) * 8 + file_diff as u64)) != 0;
    }

    if rank_diff == -1 && file_diff == 0 {
        return (board & (1 << (from_rank - 1) * 8 + file_diff as u64)) != 0;
    }

    if rank_diff == -1 && file_diff == 1 {
        return (white_board & (1 << (from_rank - 1) * 8 + file_diff as u64)) != 0;
    }

    if rank_diff == -1 && file_diff == -1 {
        return (white_board & (1 << (from_rank - 1) * 8 + file_diff as u64)) != 0;
    }

    false
}

pub fn is_king_move_blocked(from: u64, to: u64, color: bool, board: u64, white_board: u64, black_board: u64) -> bool {
    let (from_rank, rank_diff, file_diff) = basic_position_check(from, to).unwrap();

    // TODO check if the king is not in check

    false
}