

pub struct Bitboard {
    pub board: u64,
}

impl Bitboard {

    pub fn new() -> Bitboard {
        Bitboard {
            board: 0,
        }
    }

    pub fn from(board: u64) -> Bitboard {
        Bitboard {
            board,
        }
    }

    pub fn set_square(&mut self, square: usize) {
        self.board |= 1 << square;
    }

    pub fn clear_square(&mut self, square: usize) {
        self.board &= !(1 << square);
    }

    pub fn get_square(&self, square: usize) -> bool {
        (self.board & (1 << square)) != 0
    }

    pub fn get_board(&self) -> u64 {
        self.board
    }

    pub fn set_board(&mut self, board: u64) {
        self.board = board;
    }

    pub fn get_num_squares(&self) -> usize {
        self.board.count_ones() as usize
    }
}