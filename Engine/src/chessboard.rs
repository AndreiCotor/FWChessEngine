use crate::bitboard::Bitboard;

const BOARD_SIZE: usize = 8;
const NUM_SQUARES: usize = BOARD_SIZE * BOARD_SIZE;


pub struct Player {
    pub color: bool,
    pub pieces: Bitboard,
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queen: Bitboard,
    pub king: Bitboard,
}

pub struct Chessboard {
    pub board: Bitboard,
    pub white: Player,
    pub black: Player,
}