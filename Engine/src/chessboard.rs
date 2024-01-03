use crate::bitboard::{Bitboard};
use crate::constants::BOARD_SIZE;
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

    pub fn perform_move(&mut self, from: &str, to: &str, color: bool) {
        let from = Chessboard::convert_square_to_index(from);
        let to = Chessboard::convert_square_to_index(to);

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
