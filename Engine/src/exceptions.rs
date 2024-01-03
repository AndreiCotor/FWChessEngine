
#[derive(Debug)]
pub enum BitboardError {
    InvalidSquare,
    SquareOccupied,
    SquareEmpty,
}

#[derive(Debug)]
pub enum PieceError {
    OutOfBounds,
    InvalidMove,
    NoPiece,
}