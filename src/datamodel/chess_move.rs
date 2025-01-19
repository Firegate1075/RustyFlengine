use crate::datamodel::enums::piece_type::PieceType;
use crate::datamodel::field::Field;

pub struct ChessMove {
    from_field: Field,
    to_field: Field,

    /// Contains the type to promote to if the move is a promotion. Else the value is None
    promote_to: Option<PieceType>,

}

impl ChessMove {
    pub fn new(from_field: Field, to_field: Field, promote_to: Option<PieceType>) -> Self {
        ChessMove {
            from_field,
            to_field,
            promote_to,
        }
    }

    pub fn from_field(&self) -> &Field {
        &self.from_field
    }

    pub fn to_field(&self) -> &Field {
        &self.to_field
    }

    pub fn promote_to(&self) -> &Option<PieceType> {
        &self.promote_to
    }

    pub fn set_from_field(&mut self, from_field: Field) {
        self.from_field = from_field;
    }

    pub fn set_to_field(&mut self, to_field: Field) {
        self.to_field = to_field;
    }

    pub fn set_promote_to(&mut self, promote_to: Option<PieceType>) {
        self.promote_to = promote_to;
    }
}