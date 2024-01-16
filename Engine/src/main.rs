#![allow(dead_code)]

use std::io;
use crate::chessboard::chessboard::Chessboard;
use crate::chessboard::player::PlayerColor;
use crate::min_max::min_max::get_best_move;

mod constants;
mod exceptions;
mod chessboard;
mod tests;
mod min_max;
mod evaluator;

// convention: Computer plays white
fn main() {
    let mut chessboard = Chessboard::new();

    let mut player_color = PlayerColor::White;
    while !chessboard.is_finished() {
        chessboard.print_board();

        match player_color {
            PlayerColor::White => {
                println!("Computer moves...");
                let chessboard_copy = chessboard.clone();
                let best_move = get_best_move(&chessboard_copy);
                println!("{:?}", best_move);

                let from = Chessboard::convert_index_to_square(best_move.0);
                let to = Chessboard::convert_index_to_square(best_move.1);

                chessboard.perform_move(&from, &to, PlayerColor::White).unwrap();
                player_color = PlayerColor::Black
            }
            PlayerColor::Black => {
                loop {
                    let mut from = String::new();
                    let mut to = String::new();

                    println!("Enter the from position:");
                    io::stdin().read_line(&mut from).expect("Failed to read line");

                    println!("Enter the to position:");
                    io::stdin().read_line(&mut to).expect("Failed to read line");

                    let from = from.trim();
                    let to = to.trim();

                    if chessboard.perform_move(from, to, PlayerColor::Black).is_ok() {
                        player_color = PlayerColor::White;
                        break;
                    }
                    else {
                        println!("Invalid move! Try again");
                    }
                }
            }
        }
    }
    //chessboard.perform_move()
}
