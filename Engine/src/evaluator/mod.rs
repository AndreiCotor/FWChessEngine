use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::constants::NUM_SQUARES;
use crate::chessboard::piece::{ PieceType};

const PAWN_VALUE: i64 = 1;
const KNIGHT_VALUE: i64 = 3;
const BISHOP_VALUE: i64 = 3;
const ROOK_VALUE: i64 = 5;
const QUEEN_VALUE: i64 = 9;
const KING_VALUE: i64 = 0;
const NONE_VALUE: i64 = 0;
const WHITE_ADVANTAGE: i64 = 10;


pub fn evaluate(state: & Chessboard, player_color: PlayerColor) -> i64 {
    let white_material = calculate_material(state, PlayerColor::White) + WHITE_ADVANTAGE;
    let black_material = calculate_material(state, PlayerColor::Black);
    // println!("{} {}", white_material, black_material);

    match player_color {
        PlayerColor::White => white_material - black_material,
        PlayerColor::Black => black_material - white_material,
    }
}

    fn calculate_material(state: &Chessboard, player_color: PlayerColor) -> i64 {

        let mut material_value = 0;

        for index in 0..NUM_SQUARES {
            let piece_type = state.get_piece_type_color(player_color, index);

            material_value += match piece_type {
                PieceType::Pawn => PAWN_VALUE,
                PieceType::Knight => KNIGHT_VALUE,
                PieceType::Bishop => BISHOP_VALUE,
                PieceType::Rook => ROOK_VALUE,
                PieceType::Queen => QUEEN_VALUE,
                PieceType::King => KING_VALUE,
                PieceType::None => NONE_VALUE,
            };
            
        }

        material_value
    }
