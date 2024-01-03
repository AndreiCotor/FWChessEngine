use crate::bitboard::{Bitboard};
use crate::constants::BOARD_SIZE;
use crate::exceptions::{BitboardError, PieceError};
use crate::piece::PieceType;

#[derive(Debug, Clone)]
pub struct Player {
    pub color: bool, // true = white, false = black
    pub pieces: Bitboard,
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queen: Bitboard,
    pub king: Bitboard,
}

impl Player {

    pub fn new(color: bool) -> Player {

        let mut pawns: Bitboard = Bitboard::new();
        for i in 0..BOARD_SIZE {
            pawns.set_square(i + if color { 8 } else { 48 });
        }

        let mut knights: Bitboard = Bitboard::new();
        knights.set_square(1 + if color { 0 } else { 56 });
        knights.set_square(6 + if color { 0 } else { 56 });

        let mut bishops: Bitboard = Bitboard::new();
        bishops.set_square(2 + if color { 0 } else { 56 });
        bishops.set_square(5 + if color { 0 } else { 56 });

        let mut rooks: Bitboard = Bitboard::new();
        rooks.set_square(0 + if color { 0 } else { 56 });
        rooks.set_square(7 + if color { 0 } else { 56 });

        let mut queen: Bitboard = Bitboard::new();
        queen.set_square(3 + if color { 0 } else { 56 });

        let mut king: Bitboard = Bitboard::new();
        king.set_square(4 + if color { 0 } else { 56 });

        let mut pieces: Bitboard = Bitboard::new();
        pieces.set_board(
                 pawns.get_board()
                | knights.get_board()
                | bishops.get_board()
                | rooks.get_board()
                | queen.get_board()
                | king.get_board()
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
            },
            Ok(PieceType::Knight) => {
                self.knights.clear_square(from);
                self.knights.set_square(to);
            },
            Ok(PieceType::Bishop) => {
                self.bishops.clear_square(from);
                self.bishops.set_square(to);
            },
            Ok(PieceType::Rook) => {
                self.rooks.clear_square(from);
                self.rooks.set_square(to);
            },
            Ok(PieceType::Queen) => {
                self.queen.clear_square(from);
                self.queen.set_square(to);
            },
            Ok(PieceType::King) => {
                self.king.clear_square(from);
                self.king.set_square(to);
            },
            Err(_) => return Err(BitboardError::PieceNotFound),
        }

        Ok(())
    }

    pub fn update_table_after_opponent_move(&mut self, to: u64) -> Result<(), BitboardError> {
        self.pieces.clear_square(to);

        match Player::get_piece_type(self, to) {
            Ok(PieceType::Pawn) => {
                self.pawns.clear_square(to);
            },
            Ok(PieceType::Knight) => {
                self.knights.clear_square(to);
            },
            Ok(PieceType::Bishop) => {
                self.bishops.clear_square(to);
            },
            Ok(PieceType::Rook) => {
                self.rooks.clear_square(to);
            },
            Ok(PieceType::Queen) => {
                self.queen.clear_square(to);
            },
            Ok(PieceType::King) => {
                self.king.clear_square(to);
            },
            Err(_) => (),
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

        Err(PieceError::NoPiece)
    }

    pub fn has_piece_on(&self, position: u64) -> bool {
        self.pieces.get_square(position)
    }

    pub fn has_king_around(&self, position: u64) -> bool {
        let mut king = self.king.get_board();
        king &= !(1 << position);

        let king_board = Bitboard::from(king);
        king_board.get_num_squares() > 0
    }
}