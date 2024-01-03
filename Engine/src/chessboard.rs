use crate::bitboard::Bitboard;

const BOARD_SIZE: usize = 8;
const NUM_SQUARES: usize = BOARD_SIZE * BOARD_SIZE;


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
}

pub struct Chessboard {
    pub board: Bitboard,
    pub white: Player,
    pub black: Player,
}

impl Chessboard {

    pub fn new() -> Chessboard {
        let white = Player::new(true);
        let black = Player::new(false);

        let mut board= Bitboard::new();
        board.set_board(white.pieces.get_board() | black.pieces.get_board());

        Chessboard {
            board,
            white,
            black,
        }
    }

    pub fn get_board(&self) -> u64 {
        self.board.get_board()
    }

    pub fn get_white_board(&self) -> u64 {
        self.white.pieces.get_board()
    }

    pub fn get_black_board(&self) -> u64 {
        self.black.pieces.get_board()
    }

    // pub fn get_num_squares(&self) -> usize {
    //     self.board.get_num_squares()
    // }
    //
    // pub fn get_square(&self, square: usize) -> bool {
    //     self.board.get_square(square)
    // }
    //
    // pub fn set_square(&mut self, square: usize) {
    //     self.board.set_square(square);
    // }
    //
    // pub fn clear_square(&mut self, square: usize) {
    //     self.board.clear_square(square);
    // }

    pub fn perform_move(&mut self, from: &str, to: &str, color: bool) {
        let from = Chessboard::convert_square_to_index(from);
        let to = Chessboard::convert_square_to_index(to);

        // TODO: Check if move is valid
        if (color) {
            self.white.pieces.clear_square(from);
            self.white.pieces.set_square(to);
        } else {
            self.black.pieces.clear_square(from);
            self.black.pieces.set_square(to);
        }
    }

    fn convert_square_to_index(square: &str) -> usize {
        let mut chars = square.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();
        let file = file as usize - 'a' as usize;
        let rank = rank as usize - '1' as usize;
        file + rank * BOARD_SIZE
    }

    fn convert_index_to_square(index: usize) -> String {
        let file = index % BOARD_SIZE;
        let rank = index / BOARD_SIZE;
        let file = (file as u8 + 'a' as u8) as char;
        let rank = (rank as u8 + '1' as u8) as char;
        format!("{}{}", file, rank)
    }
}
