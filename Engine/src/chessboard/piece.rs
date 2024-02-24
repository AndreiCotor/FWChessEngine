use crate::chessboard::bitboard::Bitboard;
use crate::chessboard::chessboard::Chessboard;
use crate::constants::{BOARD_SIZE, NUM_SQUARES};
use crate::exceptions::{MoveError, PieceError};
use crate::chessboard::player::{Player, PlayerColor};

#[derive(PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

pub fn is_pawn_move_valid(from: u64, to: u64, color: PlayerColor) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (from_rank, rank_diff, file_diff) = result.unwrap();

    match color {
        PlayerColor::White => is_pawn_move_valid_for_white(from_rank, rank_diff, file_diff),
        PlayerColor::Black => is_pawn_move_valid_for_black(from_rank, rank_diff, file_diff),
    }
}

fn is_pawn_move_valid_for_white(
    from_rank: u64,
    rank_diff: i128,
    file_diff: i128,
) -> Result<(), PieceError> {
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

fn is_pawn_move_valid_for_black(
    from_rank: u64,
    rank_diff: i128,
    file_diff: i128,
) -> Result<(), PieceError> {
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
        || (rank_diff == -1 && file_diff == -2)
    {
        return Ok(());
    }

    Err(PieceError::InvalidMove)
}

pub fn is_bishop_move_valid(from: u64, to: u64, board: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff == file_diff || rank_diff == -file_diff {
        let bishop_moves = get_bishop_moves(from, board);
        if (bishop_moves & (1 << to)) != 0 { // bishop can move to the square (is not blocked)
            return Ok(());
        }
    }

    Err(PieceError::InvalidMove)
}

pub fn is_rook_move_valid(from: u64, to: u64, board: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff == 0 || file_diff == 0 {
        let rook_moves = get_rook_moves(from, board);
        if (rook_moves & (1 << to)) != 0 { // rook can move to the square (is not blocked)
            return Ok(());
        }
    }

    Err(PieceError::InvalidMove)
}

pub fn is_queen_move_valid(from: u64, to: u64, board: u64) -> Result<(), PieceError> {
    let result = basic_position_check(from, to);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    let (_, rank_diff, file_diff) = result.unwrap();

    if rank_diff == 0 || file_diff == 0 || rank_diff == file_diff || rank_diff == -file_diff {
        let queen_moves = get_queen_moves(from, board);
        if (queen_moves & (1 << to)) != 0 { // queen can move to the square (is not blocked)
            return Ok(());
        }
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

    if from == 4 && (to == 2 || to == 6) {
        return Ok(()); // castling white
    }

    if from == 60 && (to == 58 || to == 62) {
        return Ok(()); // castling black
    }

    Err(PieceError::InvalidMove)
}

fn basic_position_check(from: u64, to: u64) -> Result<(u64, i128, i128), PieceError> {
    if (from == to) || (from >= NUM_SQUARES) || (to >= NUM_SQUARES) {
        return Err(PieceError::InvalidMove);
    }

    let from_rank = from / 8;
    let to_rank = to / 8;

    let from_file = from % 8;
    let to_file = to % 8;

    if from_rank >= BOARD_SIZE || to_rank >= BOARD_SIZE {
        return Err(PieceError::OutOfBounds);
    }

    let to_rank: i128 = to_rank as i128;
    let to_file: i128 = to_file as i128;
    let rank_diff: i128 = to_rank - from_rank as i128;
    let file_diff: i128 = to_file - from_file as i128;

    Ok((from_rank, rank_diff, file_diff))
}

pub fn check_pawn_move_blocked(
    from: u64,
    to: u64,
    color: PlayerColor,
    board: Bitboard,
    white_board: Player,
    black_board: Player,
) -> bool {
    let (from_rank, rank_diff, file_diff) = basic_position_check(from, to).unwrap();

    match color {
        PlayerColor::White => check_pawn_move_blocked_for_white(
            from_rank,
            rank_diff,
            file_diff,
            board,
            white_board,
            black_board,
            from,
            to,
        ),
        PlayerColor::Black => check_pawn_move_blocked_for_black(
            from_rank,
            rank_diff,
            file_diff,
            board,
            white_board,
            black_board,
            from,
            to,
        ),
    }
}

fn check_pawn_move_blocked_for_white(
    from_rank: u64,
    rank_diff: i128,
    file_diff: i128,
    board: Bitboard,
    white_board: Player,
    mut black_board: Player,
    from: u64,
    to: u64,
) -> bool {
    //  account for en passant and promotion and capture

    if white_board.has_piece_on(to) {
        return true;
    }

    if from_rank == 1 && rank_diff == 2 && file_diff == 0 {
        return !board.is_square_empty(from + BOARD_SIZE)
            || !board.is_square_empty(from + 2 * BOARD_SIZE);
    }

    if rank_diff == 1 && file_diff == 0 {
        return !board.is_square_empty(from + BOARD_SIZE);
    }

    if rank_diff == 1 && file_diff == 1 {
        return !white_board.pawns.is_square_empty(from + 9)
            || black_board
                .get_piece_type(from + 9)
                .unwrap()
                .eq(&PieceType::King);
    }

    if rank_diff == 1 && file_diff == -1 {
        return !white_board.pawns.is_square_empty(from + 7)
            || black_board
                .get_piece_type(from + 7)
                .unwrap()
                .eq(&PieceType::King);
    }

    false
}

fn check_pawn_move_blocked_for_black(
    from_rank: u64,
    rank_diff: i128,
    file_diff: i128,
    board: Bitboard,
    mut white_board: Player,
    black_board: Player,
    from: u64,
    to: u64,
) -> bool {
    if black_board.has_piece_on(to) {
        return true;
    }

    if from_rank == 6 && rank_diff == -2 && file_diff == 0 {
        return !board.is_square_empty(from - BOARD_SIZE)
            || !board.is_square_empty(from - 2 * BOARD_SIZE);
    }

    if rank_diff == -1 && file_diff == 0 {
        return !board.is_square_empty(from - BOARD_SIZE);
    }

    if rank_diff == -1 && file_diff == 1 {
        return !black_board.pawns.is_square_empty(from - 7)
            || white_board
                .get_piece_type(from - 7)
                .unwrap()
                .eq(&PieceType::King);
    }

    if rank_diff == -1 && file_diff == -1 {
        return !black_board.pawns.is_square_empty(from - 9)
            || white_board
                .get_piece_type(from - 9)
                .unwrap()
                .eq(&PieceType::King);
    }

    false
}

pub fn is_king_move_blocked(
    to: u64,
    color: PlayerColor,
    board: u64,
    white_board: Player,
    black_board: Player,
) -> bool {
    match color {
        PlayerColor::White => check_king_in_check(to, board, black_board),
        PlayerColor::Black => check_king_in_check(to, board, white_board),
    }
}

pub fn check_king_in_check(
    king_pos: u64,
    board: u64,
    opponent: Player,
) -> bool {
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
            opponent_pawn_moves |= get_pawn_attack_moves(i, opponent.color);
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

pub fn get_pawn_attack_moves(pos: u64, color: PlayerColor) -> u64 {
    let mut moves = 0;

    match color {
        PlayerColor::White => {
            if pos % 8 != 0 && pos < 56 {
                moves |= 1 << (pos + 7);
            }

            if pos % 8 != 7 && pos < 56 {
                moves |= 1 << (pos + 9);
            }
        }
        PlayerColor::Black => {
            if pos % 8 != 0 && pos > 7 {
                moves |= 1 << (pos - 9);
            }

            if pos % 8 != 7 && pos > 7 {
                moves |= 1 << (pos - 7);
            }
        }
    }

    moves
}

pub fn get_pawn_moves(pos: u64, color: PlayerColor, board: u64) -> u64 {
    let mut moves = 0;

    match color {
        // check if the pawn is not on the last rank
        // check if the pawn can move forward 2 squares
        PlayerColor::White => {
            if pos < 56 {
                moves |= 1 << (pos + 8);
            }
            if (pos / 8) == 1 && (board & (1 << (pos + 8)) == 0) {
                moves |= 1 << (pos + 16);
            }
        }
        PlayerColor::Black => {
            if pos > 7 {
                moves |= 1 << (pos - 8);
            }
            if (pos / 8) == 6 && (board & (1 << (pos - 8)) == 0) {
                moves |= 1 << (pos - 16);
            }
        }
    }

    if pos % 8 != 0 {
        // check if the pawn is not on the left file
        match color {
            PlayerColor::White => {
                if pos < 56 {
                    moves |= 1 << (pos + 7);
                }
            }
            PlayerColor::Black => {
                if pos > 7 {
                    moves |= 1 << (pos - 9);
                }
            }
        }
    }

    if pos % 8 != 7 {
        // check if the pawn is not on the right file
        match color {
            PlayerColor::White => {
                if pos < 56 {
                    moves |= 1 << (pos + 9);
                }
            }
            PlayerColor::Black => {
                if pos > 7 {
                    moves |= 1 << (pos - 7);
                }
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

    let pos: i128 = pos as i128;

    let mut i = pos + 9;
    while i < NUM_SQUARES as i128 && i % 8 != 0 {
        moves |= 1 << i;
        if (board & (1 << i)) != 0 {
            break;
        }
        i += 9;
    }

    i = pos + 7;
    while i < NUM_SQUARES as i128 && i % 8 != 7 {
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

    let pos: i128 = pos as i128;

    let mut i: i128 = pos + 8;
    while i < NUM_SQUARES as i128 {
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
    while i < NUM_SQUARES as i128 && i % 8 != 0 {
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

// special cases
// 1. en passant

pub fn pawn_moves_diagonally(from: u64, to: u64) -> bool {
    if from > to {
        return from - to == 7 || from - to == 9;
    }

    to - from == 7 || to - from == 9
}

pub fn pawn_does_not_capture(to: u64, white_board: Player, black_board: Player) -> bool {
    !white_board.has_piece_on(to) && !black_board.has_piece_on(to)
}

pub fn pawn_does_en_passant_correctly(
    from: u64,
    to: u64,
    color: PlayerColor,
    white_pawn_board: Player,
    black_pawn_board: Player,
    board: Bitboard,
) -> bool {
    match color {
        PlayerColor::White => {
            check_white_pawn_does_en_passant(from, to, white_pawn_board, black_pawn_board, board)
        }
        PlayerColor::Black => {
            check_black_pawn_does_en_passant(from, to, white_pawn_board, black_pawn_board, board)
        }
    }
}

fn check_white_pawn_does_en_passant(
    from: u64,
    to: u64,
    white_pawn_board: Player,
    black_pawn_board: Player,
    board: Bitboard,
) -> bool {
    if (from / 8 != 4) || (to / 8 != 5) {
        return false;
    }

    if to - from != 7 && to - from != 9 {
        return false;
    }

    if !white_pawn_board.has_piece_on(from) || !board.is_square_empty(to) {
        return false;
    }

    if to - from == 7 {
        if !black_pawn_board.has_piece_on(from - 1) {
            return false;
        }
    } else if !black_pawn_board.has_piece_on(from + 1) {
        return false;
    }

    true
}

fn check_black_pawn_does_en_passant(
    from: u64,
    to: u64,
    white_pawn_board: Player,
    black_pawn_board: Player,
    board: Bitboard,
) -> bool {
    if (from / 8 != 3) || (to / 8 != 2) {
        return false;
    }

    if from - to != 7 && from - to != 9 {
        return false;
    }

    if !black_pawn_board.has_piece_on(from) || !board.is_square_empty(to) {
        return false;
    }

    if from - to == 7 {
        if !white_pawn_board.has_piece_on(from + 1) {
            return false;
        }
    } else if !white_pawn_board.has_piece_on(from - 1) {
        return false;
    }

    true
}

// 2. castling
pub fn is_a_castling_move(from: u64, to: u64, color: PlayerColor) -> bool {
    match color {
        PlayerColor::White => is_white_castling_move(from, to),
        PlayerColor::Black => is_black_castling_move(from, to),
    }
}

fn is_white_castling_move(from: u64, to: u64) -> bool {
    from == 4 && (to == 2 || to == 6)
}

fn is_black_castling_move(from: u64, to: u64) -> bool {
    from == 60 && (to == 58 || to == 62)
}

pub fn king_does_castling_correctly(
    from: u64,
    to: u64,
    color: PlayerColor,
    board: Bitboard,
    white_board: Player,
    black_board: Player,
) -> bool {
    match color {
        PlayerColor::White => {
            white_king_does_castling_correctly(from, to, board, white_board, black_board)
        }
        PlayerColor::Black => {
            black_king_does_castling_correctly(from, to, board, white_board, black_board)
        }
    }
}

fn white_king_does_castling_correctly(
    from: u64,
    to: u64,
    board: Bitboard,
    white_board: Player,
    black_board: Player,
) -> bool {
    if from != 4 || (to != 2 && to != 6) {
        return false;
    }

    if !white_board.has_piece_on(from) || !board.is_square_empty(to) {
        return false;
    }

    if white_board.has_king_moved || white_board.has_king_been_in_check {
        return false;
    }

    if to == 2 {
        if white_board.has_left_rook_moved {
            return false;
        }

        if white_board.rooks.is_square_empty(0)
            || !board.is_square_empty(1)
            || !board.is_square_empty(2)
            || !board.is_square_empty(3)
        {
            return false;
        }

        if check_king_in_check(
            4,
            board.get_board(),
            black_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            3,
            board.get_board(),
            black_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            2,
            board.get_board(),
            black_board,
        ) {
            return false;
        }
    } else {
        if white_board.has_right_rook_moved {
            return false;
        }

        if white_board.rooks.is_square_empty(7)
            || !board.is_square_empty(5)
            || !board.is_square_empty(6)
        {
            return false;
        }

        if check_king_in_check(
            4,
            board.get_board(),
            black_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            5,
            board.get_board(),
            black_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            6,
            board.get_board(),
            black_board,
        ) {
            return false;
        }
    }

    true
}

fn black_king_does_castling_correctly(
    from: u64,
    to: u64,
    board: Bitboard,
    white_board: Player,
    black_board: Player,
) -> bool {
    if from != 60 || (to != 58 && to != 62) {
        return false;
    }

    if !black_board.has_piece_on(from) || !board.is_square_empty(to) {
        return false;
    }

    if black_board.has_king_moved || black_board.has_king_been_in_check {
        return false;
    }

    if to == 58 {
        if black_board.has_left_rook_moved {
            return false;
        }

        if black_board.rooks.is_square_empty(56)
            || !board.is_square_empty(57)
            || !board.is_square_empty(58)
            || !board.is_square_empty(59)
        {
            return false;
        }

        if check_king_in_check(
            60,
            board.get_board(),
            white_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            59,
            board.get_board(),
            white_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            58,
            board.get_board(),
            white_board,
        ) {
            return false;
        }
    } else {
        if black_board.has_right_rook_moved {
            return false;
        }

        if black_board.rooks.is_square_empty(63)
            || !board.is_square_empty(61)
            || !board.is_square_empty(62)
        {
            return false;
        }

        if check_king_in_check(
            60,
            board.get_board(),
            white_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            61,
            board.get_board(),
            white_board.clone(),
        ) {
            return false;
        }

        if check_king_in_check(
            62,
            board.get_board(),
            white_board,
        ) {
            return false;
        }
    }

    true
}

pub fn is_small_castling(from: u64, to: u64, color: PlayerColor) -> bool {
    match color {
        PlayerColor::White => from == 4 && to == 6,
        PlayerColor::Black => from == 60 && to == 62,
    }
}

pub fn is_big_castling(from: u64, to: u64, color: PlayerColor) -> bool {
    match color {
        PlayerColor::White => from == 4 && to == 2,
        PlayerColor::Black => from == 60 && to == 58,
    }
}

// 3. promotion

pub fn pawn_promotes(from: u64, to: u64, player_color: PlayerColor) -> bool {
    match player_color {
        PlayerColor::White => from / 8 == 6 && to - from == BOARD_SIZE,
        PlayerColor::Black => from / 8 == 1 && from - to == BOARD_SIZE,
    }
}

pub fn pawn_promotes_correctly(to: u64, board: Bitboard) -> bool {
    board.is_square_empty(to)
}

// 4. king is in check
pub fn king_is_in_check(
    mut chessboard: Chessboard,
    from: u64,
    to: u64,
    color: PlayerColor,
) -> Result<bool, MoveError> {
    let move_result = match color {
        PlayerColor::White => chessboard.white.make_move(from, to),
        PlayerColor::Black => chessboard.black.make_move(from, to),
    };

    if move_result.is_err() {
        return Err(MoveError::InvalidMove);
    }

    let capture_piece_if_exists = match color {
        PlayerColor::White => chessboard.black.update_table_after_opponent_move(to),
        PlayerColor::Black => chessboard.white.update_table_after_opponent_move(to),
    };

    if capture_piece_if_exists.is_err() {
        return Err(MoveError::InvalidMove);
    }

    match color {
        PlayerColor::White => Ok(check_king_in_check(
            chessboard.white.king.get_board().trailing_zeros() as u64,
            chessboard.get_board(),
            chessboard.black.clone(),
        )),

        PlayerColor::Black => Ok(check_king_in_check(
            chessboard.black.king.get_board().trailing_zeros() as u64,
            chessboard.get_board(),
            chessboard.white.clone(),
        )),
    }
}
