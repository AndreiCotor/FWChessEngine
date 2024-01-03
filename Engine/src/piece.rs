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

pub fn is_king_move_blocked(to: u64, color: bool, board: u64, white_board: Player, black_board: Player) -> bool {

    if color {
        check_king_in_check(to, board, black_board, color)
    } else {
        check_king_in_check(to, board, white_board, color)
    }
}

fn check_king_in_check(king_pos: u64, board: u64, opponent: Player, color: bool) -> bool {

    let mut opponent_pieces = opponent.pieces.get_board();

    let mut opponent_pawns = opponent.pawns.get_board();
    let mut opponent_knights = opponent.knights.get_board();
    let mut opponent_bishops = opponent.bishops.get_board();
    let mut opponent_rooks = opponent.rooks.get_board();
    let mut opponent_queen = opponent.queen.get_board();
    let mut opponent_king = opponent.king.get_board();

    opponent_pieces &= !(1 << king_pos);

    opponent_pawns &= opponent_pieces;
    opponent_knights &= opponent_pieces;
    opponent_bishops &= opponent_pieces;
    opponent_rooks &= opponent_pieces;
    opponent_queen &= opponent_pieces;
    opponent_king &= opponent_pieces;

    let mut opponent_pawn_moves = 0;
    let mut opponent_knight_moves = 0;
    let mut opponent_bishop_moves = 0;
    let mut opponent_rook_moves = 0;
    let mut opponent_queen_moves = 0;
    let mut opponent_king_moves = 0;

    for i in 0..NUM_SQUARES {
        if (opponent_pawns & (1 << i)) != 0 {
            opponent_pawn_moves |= get_pawn_moves(i, color);
        }

        if (opponent_knights & (1 << i)) != 0 {
            opponent_knight_moves |= get_knight_moves(i);
        }

        if (opponent_bishops & (1 << i)) != 0 {
            opponent_bishop_moves |= get_bishop_moves(i, board);
        }

        if (opponent_rooks & (1 << i)) != 0 {
            opponent_rook_moves |= get_rook_moves(i, board);
        }

        if (opponent_queen & (1 << i)) != 0 {
            opponent_queen_moves |= get_queen_moves(i, board);
        }

        if (opponent_king & (1 << i)) != 0 {
            opponent_king_moves |= get_king_moves(i);
        }
    }

    if (opponent_pawn_moves & (1 << king_pos)) != 0 {
        return true;
    }

    if (opponent_knight_moves & (1 << king_pos)) != 0 {
        return true;
    }

    if (opponent_bishop_moves & (1 << king_pos)) != 0 {
        return true;
    }

    if (opponent_rook_moves & (1 << king_pos)) != 0 {
        return true;
    }

    if (opponent_queen_moves & (1 << king_pos)) != 0 {
        return true;
    }

    if (opponent_king_moves & (1 << king_pos)) != 0 {
        return true;
    }

    false
}

fn get_pawn_moves(pos: u64, color: bool) -> u64 {

    let mut moves = 0;

    if color {
        if pos < 56 {
            moves |= 1 << (pos + 8);
        }
    } else {
        if pos > 7 {
            moves |= 1 << (pos - 8);
        }
    }

    if pos % 8 != 0 {
        if color {
            if pos < 56 {
                moves |= 1 << (pos + 7);
            }
        } else {
            if pos > 7 {
                moves |= 1 << (pos - 9);
            }
        }
    }

    if pos % 8 != 7 {
        if color {
            if pos < 56 {
                moves |= 1 << (pos + 9);
            }
        } else {
            if pos > 7 {
                moves |= 1 << (pos - 7);
            }
        }
    }

    moves
}

pub fn get_knight_moves(pos: u64) -> u64 {
    let mut moves = 0;

    if pos % 8 > 1 {
        if pos < 56 {
            moves |= 1 << (pos + 6);
        }

        if pos > 7 {
            moves |= 1 << (pos - 10);
        }
    }

    if pos % 8 > 0 {
        if pos < 48 {
            moves |= 1 << (pos + 15);
        }

        if pos > 15 {
            moves |= 1 << (pos - 17);
        }
    }

    if pos % 8 < 6 {
        if pos < 56 {
            moves |= 1 << (pos + 10);
        }

        if pos > 7 {
            moves |= 1 << (pos - 6);
        }
    }

    if pos % 8 < 7 {
        if pos < 48 {
            moves |= 1 << (pos + 17);
        }

        if pos > 15 {
            moves |= 1 << (pos - 15);
        }
    }

    moves
}

pub fn get_bishop_moves(pos: u64, board: u64) -> u64 {
    let mut moves = 0;

    let mut i = pos + 9;
    while i < NUM_SQUARES && i % 8 != 0 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i += 9;
    }

    i = pos + 7;
    while i < NUM_SQUARES && i % 8 != 7 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i += 7;
    }

    i = pos - 9;
    while i >= 0 && i % 8 != 7 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i -= 9;
    }

    i = pos - 7;
    while i >= 0 && i % 8 != 0 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i -= 7;
    }

    moves
}

pub fn get_rook_moves(pos: u64, board: u64) -> u64 {
    let mut moves = 0;

    let mut i = pos + 8;
    while i < NUM_SQUARES {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i += 8;
    }

    i = pos - 8;
    while i >= 0 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i -= 8;
    }

    i = pos + 1;
    while i < NUM_SQUARES && i % 8 != 0 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i += 1;
    }

    i = pos - 1;
    while i >= 0 && i % 8 != 7 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i -= 1;
    }

    moves
}

pub fn get_queen_moves(pos: u64, board: u64) -> u64 {
    get_bishop_moves(pos, board) | get_rook_moves(pos, board)
}

pub fn get_king_moves(pos: u64) -> u64 {
    let mut moves = 0;

    if pos < 56 {
        moves |= 1 << (pos + 8);
    }

    if pos > 7 {
        moves |= 1 << (pos - 8);
    }

    if pos % 8 != 0 {
        if pos < 56 {
            moves |= 1 << (pos + 7);
        }

        if pos > 7 {
            moves |= 1 << (pos - 9);
        }

        moves |= 1 << (pos - 1);
    }

    if pos % 8 != 7 {
        if pos < 56 {
            moves |= 1 << (pos + 9);
        }

        if pos > 7 {
            moves |= 1 << (pos - 7);
        }

        moves |= 1 << (pos + 1);
    }

    moves
}
