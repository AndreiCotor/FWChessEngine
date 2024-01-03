
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

        assert_eq!(chessboard.perform_move("c7", "c6", false), Ok(()));
        assert_eq!(chessboard.perform_move("c6", "c5", false), Ok(()));
        assert_eq!(chessboard.perform_move("c5", "c4", false), Ok(()));
        assert_eq!(chessboard.perform_move("c4", "c3", false), Ok(()));
        assert_eq!(chessboard.perform_move("c3", "c2", false), Err(MoveError::SquareOccupied));

        assert_eq!(chessboard.perform_move("c3", "d2", true), Ok(()));
        assert_eq!(chessboard.perform_move("d2", "e1", true), Err(MoveError::SquareOccupied));
        assert_eq!(chessboard.perform_move("d2", "d1", true), Err(MoveError::InvalidMove));
        assert_eq!(chessboard.perform_move("d2", "c1", true), Ok(()));
    }
}