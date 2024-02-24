use crate::chessboard::bitboard::Bitboard;
use crate::constants::BOARD_SIZE;
use crate::exceptions::{BitboardError, PieceError};
use crate::chessboard::piece::PieceType;

#[derive(Debug, Clone, Copy)]
pub enum PlayerColor {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub color: PlayerColor,
    pub pieces: Bitboard,
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queen: Bitboard,
    pub king: Bitboard,
    pub has_left_rook_moved: bool,
    pub has_right_rook_moved: bool,
    pub has_king_moved: bool,
    pub has_king_been_in_check: bool,
}

impl Player {
    pub fn new(color: PlayerColor) -> Player {
        let mut pawns: Bitboard = Bitboard::new();
        for i in 0..BOARD_SIZE {
            pawns.set_square(
                i + match color {
                    PlayerColor::White => 8,
                    PlayerColor::Black => 48,
                },
            );
        }

        let mut knights: Bitboard = Bitboard::new();
        knights.set_square(
            1 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );
        knights.set_square(
            6 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );

        let mut bishops: Bitboard = Bitboard::new();
        bishops.set_square(
            2 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );
        bishops.set_square(
            5 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );

        let mut rooks: Bitboard = Bitboard::new();
        rooks.set_square(
            match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );
        rooks.set_square(
            7 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );

        let mut queen: Bitboard = Bitboard::new();
        queen.set_square(
            3 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );

        let mut king: Bitboard = Bitboard::new();
        king.set_square(
            4 + match color {
                PlayerColor::White => 0,
                PlayerColor::Black => 56,
            },
        );

        let mut pieces: Bitboard = Bitboard::new();
        pieces.set_board(
            pawns.get_board()
                | knights.get_board()
                | bishops.get_board()
                | rooks.get_board()
                | queen.get_board()
                | king.get_board(),
        );

        Player {
            color,
            pieces,
            pawns,
            knights,
            bishops,
            rooks,
            queen,
            king,
            has_left_rook_moved: false,
            has_right_rook_moved: false,
            has_king_moved: false,
            has_king_been_in_check: false,
        }
    }

    pub fn get_board(&self) -> u64 {
        self.pieces.get_board()
    }

    pub fn make_move(&mut self, from: u64, to: u64) -> Result<(), BitboardError> {
        self.pieces.clear_square(from);
        self.pieces.set_square(to);

        match Player::get_piece_type(self, from) {
            Ok(PieceType::Pawn) => {
                self.pawns.clear_square(from);
                self.pawns.set_square(to);
            }
            Ok(PieceType::Knight) => {
                self.knights.clear_square(from);
                self.knights.set_square(to);
            }
            Ok(PieceType::Bishop) => {
                self.bishops.clear_square(from);
                self.bishops.set_square(to);
            }
            Ok(PieceType::Rook) => {
                self.rooks.clear_square(from);
                self.rooks.set_square(to);

                match self.color {
                    PlayerColor::White => {
                        if from == 0 {
                            self.has_left_rook_moved = true;
                        } else if from == 7 {
                            self.has_right_rook_moved = true;
                        }
                    }
                    PlayerColor::Black => {
                        if from == 56 {
                            self.has_left_rook_moved = true;
                        } else if from == 63 {
                            self.has_right_rook_moved = true;
                        }
                    }
                }
            }
            Ok(PieceType::Queen) => {
                self.queen.clear_square(from);
                self.queen.set_square(to);
            }
            Ok(PieceType::King) => {
                self.king.clear_square(from);
                self.king.set_square(to);
                self.has_king_moved = true;
            }
            Ok(PieceType::None) => return Err(BitboardError::PieceNotFound),
            Err(_) => return Err(BitboardError::PieceNotFound),
        }

        Ok(())
    }

    pub fn update_table_after_opponent_move(&mut self, to: u64) -> Result<(), BitboardError> {
        self.pieces.clear_square(to);

        match Player::get_piece_type(self, to) {
            Ok(PieceType::Pawn) => {
                self.pawns.clear_square(to);
            }
            Ok(PieceType::Knight) => {
                self.knights.clear_square(to);
            }
            Ok(PieceType::Bishop) => {
                self.bishops.clear_square(to);
            }
            Ok(PieceType::Rook) => {
                self.rooks.clear_square(to);
            }
            Ok(PieceType::Queen) => {
                self.queen.clear_square(to);
            }
            Ok(PieceType::King) => {
                self.king.clear_square(to);
            }
            Err(_) => (),
            _ => {}
        }

        Ok(())
    }

    pub fn promote_pawn(
        &mut self,
        origin: u64,
        position: u64,
        piece_type: PieceType,
    ) -> Result<(), BitboardError> {
        let move_res = self.make_move(origin, position);
        if move_res.is_err() {
            return Err(BitboardError::InvalidPromotion);
        }

        match piece_type {
            PieceType::Pawn => return Err(BitboardError::InvalidPromotion),
            PieceType::King => return Err(BitboardError::InvalidPromotion),
            PieceType::Knight => {
                self.pawns.clear_square(position);
                self.knights.set_square(position);
            }
            PieceType::Bishop => {
                self.pawns.clear_square(position);
                self.bishops.set_square(position);
            }
            PieceType::Rook => {
                self.pawns.clear_square(position);
                self.rooks.set_square(position);
            }
            PieceType::Queen => {
                self.pawns.clear_square(position);
                self.queen.set_square(position);
            }
            PieceType::None => return Err(BitboardError::InvalidPromotion),
        }

        Ok(())
    }

    pub fn perform_small_castling(&mut self) -> Result<(), BitboardError> {
        match self.color {
            PlayerColor::White => {
                if self.make_move(4, 6).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
                if self.make_move(7, 5).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
            }
            PlayerColor::Black => {
                if self.make_move(60, 62).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
                if self.make_move(63, 61).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
            }
        }

        Ok(())
    }

    pub fn perform_big_castling(&mut self) -> Result<(), BitboardError> {
        match self.color {
            PlayerColor::White => {
                if self.make_move(4, 2).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
                if self.make_move(0, 3).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
            }
            PlayerColor::Black => {
                if self.make_move(60, 58).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
                if self.make_move(56, 59).is_err() {
                    return Err(BitboardError::InvalidCastling);
                }
            }
        }

        Ok(())
    }

    pub fn get_piece_type(&mut self, position: u64) -> Result<PieceType, PieceError> {
        if self.pawns.get_square(position) {
            return Ok(PieceType::Pawn);
        }

        if self.knights.get_square(position) {
            return Ok(PieceType::Knight);
        }

        if self.bishops.get_square(position) {
            return Ok(PieceType::Bishop);
        }

        if self.rooks.get_square(position) {
            return Ok(PieceType::Rook);
        }

        if self.queen.get_square(position) {
            return Ok(PieceType::Queen);
        }

        if self.king.get_square(position) {
            return Ok(PieceType::King);
        }

        Ok(PieceType::None)
    }

    pub fn has_piece_on(&self, position: u64) -> bool {
        self.pieces.get_square(position)
    }

    pub fn has_king_around(&self, position: u64) -> bool {
        let mut valid_positions: u64 = 0;
        let mid_position: i64 = position as i64;

        for i in mid_position - 1..=mid_position + 1 {
            if i < 0 || i >= BOARD_SIZE as i64 {
                continue;
            }

            for j in mid_position - 1..=mid_position + 1 {
                if j < 0 || j >= BOARD_SIZE as i64 {
                    continue;
                }

                let index = i * BOARD_SIZE as i64 + j;
                valid_positions |= 1 << index;
            }
        }

        let mut king = self.king.get_board();
        king &= valid_positions;

        let king_board = Bitboard::from(king);
        king_board.get_num_squares() > 0
    }

    // 0..64
    pub fn get_king_position_on_board(&self) -> u64 {
        self.king.get_board().trailing_zeros() as u64
    }

    pub fn set_king_in_check(&mut self, value: bool) {
        self.has_king_been_in_check = true;
    }
}
