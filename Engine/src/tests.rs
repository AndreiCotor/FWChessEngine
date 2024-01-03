
#[cfg(test)]
mod tests {

    use crate::chessboard::Chessboard;
    use crate::exceptions::MoveError;

    #[test]
    fn test_chessboard_valid_moves() {
        let mut chessboard = Chessboard::new();

        // Test white pawn moves
        assert_eq!(chessboard.perform_move("a2", "a3", true), Ok(()));
        assert_eq!(chessboard.perform_move("a3", "a4", true), Ok(()));
        assert_eq!(chessboard.perform_move("a4", "a5", true), Ok(()));
        assert_eq!(chessboard.perform_move("a5", "a6", true), Ok(()));
        assert_eq!(chessboard.perform_move("a6", "a7", true), Err(MoveError::SquareOccupied));

        // Test black pawn moves
        assert_eq!(chessboard.perform_move("a7", "a6", false), Err(MoveError::SquareOccupied));

        assert_eq!(chessboard.perform_move("b7", "b6", false), Ok(()));
        assert_eq!(chessboard.perform_move("b6", "b5", false), Ok(()));
        assert_eq!(chessboard.perform_move("b5", "b4", false), Ok(()));
        assert_eq!(chessboard.perform_move("b4", "b3", false), Ok(()));
        assert_eq!(chessboard.perform_move("b3", "b2", false), Err(MoveError::SquareOccupied));
    }
}