#[cfg(test)]
mod tests {
    use crate::chessboard::chessboard::Chessboard;
    use crate::exceptions::MoveError;
    use crate::chessboard::player::PlayerColor;

    #[test]
    fn test_chessboard_valid_moves() {
        println!(
            "\n*******************\nRunning test_chessboard_valid_moves()\n*******************\n"
        );

        let mut chessboard = Chessboard::new();

        // Test white pawn moves
        assert_eq!(
            chessboard.perform_move("a2", "a3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a3", "a4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a4", "a5", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a5", "a6", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a6", "a7", PlayerColor::White),
            Err(MoveError::InvalidMove)
        );

        // Test black pawn moves
        assert_eq!(
            chessboard.perform_move("a7", "a6", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );

        assert_eq!(
            chessboard.perform_move("c7", "c6", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("c6", "c5", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("c5", "c4", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("c4", "c3", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("c3", "c2", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );

        assert_eq!(
            chessboard.perform_move("c3", "d2", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("d2", "e1", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("d2", "d1", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("d2", "c1", PlayerColor::Black),
            Ok(())
        );

        // Test knight moves
        assert_eq!(
            chessboard.perform_move("b1", "c3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("c3", "e4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e4", "f6", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f6", "e8", PlayerColor::White),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("f6", "g8", PlayerColor::White),
            Ok(())
        );

        // Test bishop moves
        assert_eq!(
            chessboard.perform_move("c1", "a3", PlayerColor::White),
            Err(MoveError::PieceNotFound)
        );

        assert_eq!(
            chessboard.perform_move("e2", "e3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f1", "c4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("c4", "c3", PlayerColor::White),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("c4", "c5", PlayerColor::White),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("c4", "b3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("b3", "a4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a4", "d7", PlayerColor::White),
            Ok(())
        );

        // Test rook moves
        assert_eq!(
            chessboard.perform_move("a1", "a2", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a2", "a3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("a3", "a4", PlayerColor::White),
            Ok(())
        );

        // Test queen moves
        assert_eq!(
            chessboard.perform_move("d1", "d2", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("d2", "d3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("d3", "d4", PlayerColor::White),
            Ok(())
        );

        // Test king moves
        assert_eq!(
            chessboard.perform_move("e1", "e2", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e2", "e3", PlayerColor::White),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("e2", "f3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f3", "f4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f4", "f5", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f5", "g5", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("g5", "h4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("h4", "g3", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e8", "d7", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("d4", "c3", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e8", "d7", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("d7", "e6", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e6", "f5", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f5", "g4", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
    }

    #[test]
    fn test_special_moves() {
        println!("\n*******************\nRunning play_game_special_moves()\n*******************\n");

        let mut chessboard = Chessboard::new();

        // Test en passant
        assert_eq!(
            chessboard.perform_move("e2", "e4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("d7", "d5", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e4", "e5", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("b7", "b6", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e5", "d6", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("b6", "c5", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("b6", "b4", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("b6", "b5", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("h2", "h4", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("b5", "b4", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("h4", "h5", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("b4", "a3", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("b4", "c3", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );

        assert_eq!(
            chessboard.perform_move("e7", "e5", PlayerColor::Black),
            Ok(())
        );

        // Test castling
        assert_eq!(
            chessboard.perform_move("g1", "f3", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("g8", "f6", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("f1", "e2", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("f8", "e7", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e1", "g1", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("h8", "g8", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("g8", "h8", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e8", "g8", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );

        // Test promotion
        assert_eq!(
            chessboard.perform_move("e2", "d3", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e5", "e4", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e4", "e3", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e3", "e2", PlayerColor::Black),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e2", "e1", PlayerColor::Black),
            Ok(())
        );

        // Test check
        assert_eq!(
            chessboard.perform_move("d1", "e2", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("e7", "d6", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );

        assert_eq!(
            chessboard.perform_move("e2", "d1", PlayerColor::White),
            Ok(())
        );
        assert_eq!(
            chessboard.perform_move("e7", "d6", PlayerColor::Black),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("d1", "e2", PlayerColor::White),
            Ok(())
        );

        assert_eq!(
            chessboard.perform_move("d6", "c5", PlayerColor::Black),
            Err(MoveError::InvalidMove)
        );
        assert_eq!(
            chessboard.perform_move("d6", "e5", PlayerColor::Black),
            Ok(())
        );
    }
}
