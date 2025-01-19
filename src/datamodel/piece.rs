
use super::enums::color::Color;
use super::enums::piece_type::PieceType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece { color, piece_type }
    }
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_piece_type(&mut self, piece_type: PieceType) {
        self.piece_type = piece_type;
    }
}