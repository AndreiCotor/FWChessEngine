use crate::bitboard::{Bitboard};
use crate::constants::BOARD_SIZE;
use crate::piece::PieceType;
use crate::player::Player;


// Table orientation:
//   a b c d e f g h
// 8|
// 7|
// 6|
// 5|
// 4|
// 3|
// 2|8 9 ...
// 1|0 1 2 3 4 5 6 7

// convention: true = white, false = black
// white on bottom, black on top

pub struct Chessboard {
    pub white: Player,
    pub black: Player,
}

struct Piece(u64, u64, bool);

impl Chessboard {

    pub fn new() -> Chessboard {
        let white = Player::new(true);
        let black = Player::new(false);

        Chessboard {
            white,
            black,
        }
    }

    pub fn get_board(&self) -> u64 {
        self.white.pieces.get_board() | self.black.pieces.get_board()
    }

    pub fn get_white_board(&self) -> u64 {
        self.white.pieces.get_board()
    }

    pub fn get_black_board(&self) -> u64 {
        self.black.pieces.get_board()
    }

    pub fn perform_move(&mut self, from: &str, to: &str, color: bool) {
        let from = Chessboard::convert_square_to_index(from);
        let to = Chessboard::convert_square_to_index(to);

        // Validation steps:
        // 1. Check if the piece is on the board
        // 2. Check if the piece is the correct color
        // 3. Check if the move is valid for the piece
        // 4. Check if the move is blocked by another piece
        // 5. Check if the move puts the king in check

        // 1, 2
        let piece_type = if color {
            self.white.get_piece_type(from)
        } else {
            self.black.get_piece_type(from)
        };

        if piece_type.is_err() {
            println!("Invalid piece");
            return;
        }

        let piece_type = piece_type.unwrap();

        // 3
        let is_move_valid = match piece_type {
            PieceType::Pawn => Piece::is_pawn_move_valid(from, to, color),
            PieceType::Knight => Piece::is_knight_move_valid(from, to),
            PieceType::Bishop => Piece::is_bishop_move_valid(from, to),
            PieceType::Rook => Piece::is_rook_move_valid(from, to),
            PieceType::Queen => Piece::is_queen_move_valid(from, to),
            PieceType::King => Piece::is_king_move_valid(from, to),
        };

        if is_move_valid.is_err() {
            println!("Invalid move");
            return;
        }

        // 4, 5
        let is_move_blocked = match color {
            true => match piece_type {
                PieceType::Pawn => {
                    // account for en passant and promotion and capture
                    self.black.has_piece_on(to)
                        || self.black.get_piece_type(to) == Ok(PieceType::King)
                        || Piece::check_pawn_move_blocked(from, to, color, self.get_board(), self.get_white_board(), self.get_black_board())
                },
                PieceType::King => {
                    self.white.has_piece_on(to)
                        || self.black.get_piece_type(to) == Ok(PieceType::King)
                        || self.black.has_king_around(to)
                        || Piece::is_king_move_blocked(from, to, color, self.get_board(), self.get_white_board(), self.get_black_board())
                },
                _ => self.white.has_piece_on(to) || self.black.get_piece_type(to) == Ok(PieceType::King),
            },
            false => self.black.has_piece_on(to) || self.white.get_piece_type(to) == Ok(PieceType::King),
        };

        if is_move_blocked {
            println!("Move blocked");
            return;
        }

        let move_result = if color {
            self.white.make_move(from, to)
        } else {
            self.black.make_move(from, to)
        };

        match move_result {
            Ok(_) => {
                println!("Move successful");
            },
            Err(_) => {
                println!("Invalid move");
            },
        }
    }

    fn convert_square_to_index(square: &str) -> u64 {
        let mut chars = square.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();
        let file = file as u64 - 'a' as u64;
        let rank = rank as u64 - '1' as u64;
        file + rank * BOARD_SIZE
    }

    fn convert_index_to_square(index: u64) -> String {
        let file = index % BOARD_SIZE;
        let rank = index / BOARD_SIZE;
        let file = (file as u8 + 'a' as u8) as char;
        let rank = (rank as u8 + '1' as u8) as char;
        format!("{}{}", file, rank)
    }
}
