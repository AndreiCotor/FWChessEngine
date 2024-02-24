use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::chessboard::piece;

pub fn generate_next_moves(chessboard: &Chessboard, player_color: PlayerColor) -> Vec<(u64, u64)> {
    match player_color {
        PlayerColor::White => generate_white_moves(chessboard),
        PlayerColor::Black => generate_black_moves(chessboard)
    }
}

// white on bottom, black on top
fn generate_white_moves(chessboard: &Chessboard) -> Vec<(u64, u64)> {
    let mut result = vec![];

    for bit in 0..64 {
        let potential_move = if chessboard.white.pawns.get_square(bit) {
            piece::get_pawn_moves(bit, PlayerColor::White, chessboard.get_board())
        }
        else if chessboard.white.bishops.get_square(bit) {
            piece::get_bishop_moves(bit, chessboard.get_board())
        }
        else if chessboard.white.king.get_square(bit) {
            piece::get_king_moves(bit)
        }
        else if chessboard.white.queen.get_square(bit) {
            piece::get_queen_moves(bit, chessboard.get_board())
        }
        else if chessboard.white.rooks.get_square(bit) {
            piece::get_rook_moves(bit, chessboard.get_board())
        }
        else if chessboard.white.knights.get_square(bit) {
            piece::get_knight_moves(bit)
        }
        else {
            0
        };

        add_moves(bit, potential_move, chessboard, PlayerColor::White, &mut result);
    }

    result
}

fn generate_black_moves(chessboard: &Chessboard) -> Vec<(u64, u64)> {
    let mut result = vec![];

    for bit in 0..64 {
        let potential_move = if chessboard.black.pawns.get_square(bit) {
            piece::get_pawn_moves(bit, PlayerColor::Black, chessboard.get_board())
        }
        else if chessboard.black.bishops.get_square(bit) {
            piece::get_bishop_moves(bit, chessboard.get_board())
        }
        else if chessboard.black.king.get_square(bit) {
            piece::get_king_moves(bit)
        }
        else if chessboard.black.queen.get_square(bit) {
            piece::get_queen_moves(bit, chessboard.get_board())
        }
        else if chessboard.black.rooks.get_square(bit) {
            piece::get_rook_moves(bit, chessboard.get_board())
        }
        else if chessboard.black.knights.get_square(bit) {
            piece::get_knight_moves(bit)
        }
        else {
            0
        };

        add_moves(bit, potential_move, chessboard, PlayerColor::Black, &mut result);
    }

    result
}

fn add_moves(
    original: u64,
    new_position_mask: u64,
    chessboard: &Chessboard,
    player_color: PlayerColor,
    result: &mut Vec<(u64, u64)>
) {
    let original_str = Chessboard::convert_index_to_square(original);

    for bit in 0..64 {
        if ((1u64 << bit) & new_position_mask) != 0 {
            let bit_str = Chessboard::convert_index_to_square(bit);
            let mut new_chessboard = chessboard.clone();

            if new_chessboard.perform_move(&original_str, &bit_str, player_color).is_ok() {
                result.push((original, bit));
            }
        }
    }
}