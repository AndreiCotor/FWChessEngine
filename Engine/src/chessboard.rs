use crate::bitboard::Bitboard;
use crate::constants::BOARD_SIZE;
use crate::exceptions::{MoveError, PieceError};
use crate::piece::{check_pawn_does_en_passant_correctly, check_pawn_move_blocked, is_bishop_move_valid, is_king_move_blocked, is_king_move_valid, is_knight_move_valid, is_pawn_move_valid, is_queen_move_valid, is_rook_move_valid, pawn_does_not_capture, pawn_moves_diagonally, PieceType};
use crate::player::{Player, PlayerColor};


// Table orientation:
//   a b c d e f g h
// 1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
// 2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
// 3
// 4
// 5
// 6
// 7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
// 8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖

// convention: true = white, false = black
// white on bottom, black on top

pub struct Chessboard {
    pub white: Player,
    pub black: Player,
}

impl Chessboard {

    pub fn new() -> Chessboard {
        let white = Player::new(PlayerColor::White);
        let black = Player::new(PlayerColor::Black);

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

    pub fn perform_move(&mut self, from: &str, to: &str, color: PlayerColor) -> Result<(), MoveError> {
        let from = Chessboard::convert_square_to_index(from);
        let to = Chessboard::convert_square_to_index(to);

        // Validation steps:
        // 1. Check if the piece is on the board
        // 2. Check if the piece is the correct color
        // 3. Check if the move is valid for the piece
        // 4. Check if it is a special move
        // 5. Check if the move is blocked by another piece
        // 6. Check if the move puts the king in check

        // 1, 2
        let piece_type = match color {
            PlayerColor::White => self.white.get_piece_type(from),
            PlayerColor::Black => self.black.get_piece_type(from),
        };

        if piece_type.is_err() {
             return Err(MoveError::PieceNotFound);
        }

        let piece_type = piece_type.unwrap();

        if piece_type == PieceType::None {
            return Err(MoveError::PieceNotFound);
        }

        // 3
        let is_move_valid = match piece_type {
            PieceType::Pawn => is_pawn_move_valid(from, to, color),
            PieceType::Knight => is_knight_move_valid(from, to),
            PieceType::Bishop => is_bishop_move_valid(from, to),
            PieceType::Rook => is_rook_move_valid(from, to),
            PieceType::Queen => is_queen_move_valid(from, to),
            PieceType::King => is_king_move_valid(from, to),
            PieceType::None => Err(PieceError::NoPiece),
        };

        if is_move_valid.is_err() {
            return Err(MoveError::InvalidMove);
        }

        // 4
        // check if it is an en passant move
        if piece_type == PieceType::Pawn {
            if pawn_moves_diagonally(from, to) &&
                pawn_does_not_capture(to, self.white.clone(), self.black.clone()) {

                return if check_pawn_does_en_passant_correctly(from, to, color, self.white.clone(), self.black.clone(), Bitboard::from(self.get_board())) {
                    self.perform_en_passant(from, to, color)
                } else {
                    Err(MoveError::InvalidMove)
                }
            }
        }

        // check if it is a castling move
        // check if it is a promotion move
        // check if the king is in check


        // 5, 6
        let is_move_blocked = match color {
            PlayerColor::White => match piece_type {
                PieceType::Pawn => {
                    // account for en passant and promotion and capture
                    self.white.has_piece_on(to)
                        || self.black.get_piece_type(to) == Ok(PieceType::King)
                        || check_pawn_move_blocked(from, to, color, Bitboard::from(self.get_board()), self.white.clone(), self.black.clone())
                },
                PieceType::King => {
                    self.white.has_piece_on(to)
                        || self.black.get_piece_type(to) == Ok(PieceType::King)
                        || self.black.has_king_around(to)
                        || is_king_move_blocked(to, color, self.get_board(), self.white.clone(), self.black.clone())
                },
                _ => self.white.has_piece_on(to) || self.black.get_piece_type(to) == Ok(PieceType::King),
            },
            PlayerColor::Black => match piece_type {
                PieceType::Pawn => {
                    // account for en passant and promotion and capture
                    self.black.has_piece_on(to)
                        || self.white.get_piece_type(to) == Ok(PieceType::King)
                        || check_pawn_move_blocked(from, to, color, Bitboard::from(self.get_board()), self.white.clone(), self.black.clone())
                },
                PieceType::King => {
                    self.black.has_piece_on(to)
                        || self.white.get_piece_type(to) == Ok(PieceType::King)
                        || self.white.has_king_around(to)
                        || is_king_move_blocked(to, color, self.get_board(), self.white.clone(), self.black.clone())
                },
                _ => self.black.has_piece_on(to) || self.white.get_piece_type(to) == Ok(PieceType::King),
            }
        };

        if is_move_blocked {
            return Err(MoveError::InvalidMove);
        }

        let move_result = match color {
            PlayerColor::White => self.white.make_move(from, to),
            PlayerColor::Black => self.black.make_move(from, to),
        };

        if move_result.is_err() {
            return Err(MoveError::InvalidMove);
        }

        let capture_piece_if_exists = match color {
            PlayerColor::White => self.black.update_table_after_opponent_move(to),
            PlayerColor::Black => self.white.update_table_after_opponent_move(to),
        };

        if capture_piece_if_exists.is_err() {
            return Err(MoveError::InvalidMove);
        }

        println!("Moved from {} to {}: ", from, to);
        Chessboard::print_board(self);

        Ok(())
    }

    fn perform_en_passant(&mut self, from: u64, to: u64, player_color: PlayerColor) -> Result<(), MoveError> {
        let move_result = match player_color {
            PlayerColor::White => self.white.make_move(from, to),
            PlayerColor::Black => self.black.make_move(from, to),
        };

        if move_result.is_err() {
            return Err(MoveError::InvalidMove);
        }

        let capture_piece_if_exists = match player_color {
            PlayerColor::White => self.black.update_table_after_opponent_move(to - BOARD_SIZE),
            PlayerColor::Black => self.white.update_table_after_opponent_move(to + BOARD_SIZE),
        };

        if capture_piece_if_exists.is_err() {
            return Err(MoveError::InvalidMove);
        }

        println!("Moved from {} to {}: ", from, to);
        Chessboard::print_board(self);

        Ok(())
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

    fn print_board(&mut self) {
        let board = self.get_board();
        let mut board = format!("{:064b}", board);
        board = board.chars().rev().collect::<String>();
        let mut board = board.chars();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {

                let square = board.next().unwrap();
                if square == 0 as char {
                    print!(". ");
                    continue;
                }

                let index = i * BOARD_SIZE + j;
                let piece = self.white.get_piece_type(index)
                        .unwrap_or_else(|_| PieceType::None);

                match piece {
                    PieceType::Pawn => print!("♟ "),
                    PieceType::Knight => print!("♞ "),
                    PieceType::Bishop => print!("♝ "),
                    PieceType::Rook => print!("♜ "),
                    PieceType::Queen => print!("♛ "),
                    PieceType::King => print!("♚ "),
                    PieceType::None => {
                        let piece = self.black.get_piece_type(index)
                            .unwrap_or_else(|_| PieceType::None);
                        match piece {
                            PieceType::Pawn => print!("♙ "),
                            PieceType::Knight => print!("♘ "),
                            PieceType::Bishop => print!("♗ "),
                            PieceType::Rook => print!("♖ "),
                            PieceType::Queen => print!("♕ "),
                            PieceType::King => print!("♔ "),
                            PieceType::None => print!(". "),
                        }
                    }
                }
            }
            println!();
        }
    }
}
