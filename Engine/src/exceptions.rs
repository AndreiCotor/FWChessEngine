
#[derive(Debug, PartialOrd, PartialEq)]
pub enum BitboardError {
    InvalidSquare,
    SquareOccupied,
    SquareEmpty,
    PieceNotFound,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum PieceError {
    OutOfBounds,
    InvalidMove,
    NoPiece,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum MoveError {
    InvalidMove,
    InvalidPiece,
    InvalidSquare,
    SquareOccupied,
    SquareEmpty,
    KingInCheck,
    PieceNotFound,
}