
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum PieceType {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
}

impl PieceType {
    fn value(&self) -> u8 {
        match self {
            PieceType::PAWN => 1,
            PieceType::ROOK => 5,
            PieceType::BISHOP => 3,
            PieceType::KNIGHT => 3,
            PieceType::QUEEN => 9,
            PieceType::KING => 100,
        }
    }
}