use crate::bitboard::{Bitboard, BitboardError};

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
        for i in 0..crate::chessboard::BOARD_SIZE {
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

        let mut queen: Bitboard = Bitboard::new();
        queen.set_square(3 + if color { 0 } else { 56 });

        let mut king: Bitboard = Bitboard::new();
        king.set_square(4 + if color { 0 } else { 56 });

        let mut pieces: Bitboard = Bitboard::new();
        pieces.set_board(
            | pawns.get_board()
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
        //
        // if (from >= NUM_SQUARES) || (to >= NUM_SQUARES) || (from == to) || (from < 0) || (to < 0) {
        //     return Err(BitboardError::InvalidSquare);
        // }
        //
        // if !self.pieces.get_square(from) {
        //     return Err(BitboardError::SquareEmpty);
        // }
        //
        self.pieces.clear_square(from);
        self.pieces.set_square(to);

        Ok(())
    }

    pub fn update_table_after_opponent_move(&mut self, to: u64) -> Result<(), BitboardError> {
        self.pieces.clear_square(to);

        Ok(())
    }
}