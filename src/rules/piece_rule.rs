use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::color::Color;
use crate::datamodel::enums::file::File;
use crate::datamodel::enums::piece_type::PieceType;
use crate::datamodel::enums::rank::Rank;
use crate::datamodel::field::Field;

/// Implementation of the individual rules for each chess piece.
/// Also contains the methods isFieldCovered and isLegalMove.
struct PieceRule;

impl PieceRule {
    /// Returns all possible moves of a piece on the given field.
    pub fn get_legal_moves(board: &Board, field: &Field) -> Vec<ChessMove> {
        match board.get_piece(field).map(|piece| piece.piece_type()) {
            Some(PieceType::PAWN) => get_legal_moves_pawn(board, field),
            Some(PieceType::ROOK) => get_legal_moves_rook(board, field),
            Some(PieceType::KNIGHT) => get_legal_moves_knight(board, field),
            Some(PieceType::BISHOP) => get_legal_moves_bishop(board, field),
            Some(PieceType::QUEEN) => get_legal_moves_queen(board, field),
            Some(PieceType::KING) => get_legal_moves_king(board, field),
            None => Vec::new(),
        }
    }
}

fn get_legal_moves_king(board: &Board, field: &Field) -> Vec<ChessMove> {
    todo!()
}

fn get_legal_moves_queen(board: &Board, field: &Field) -> Vec<ChessMove> {
    todo!()
}

fn get_legal_moves_bishop(board: &Board, field: &Field) -> Vec<ChessMove> {
    todo!()
}

fn get_legal_moves_knight(board: &Board, field: &Field) -> Vec<ChessMove> {
    todo!()
}

fn get_legal_moves_rook(board: &Board, field: &Field) -> Vec<ChessMove> {
    todo!()
}

/// Returns all possible moves of a pawn at given position.
fn get_legal_moves_pawn(board: &Board, field: &Field) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let color: Color = board.get_piece(field).unwrap().color();
    let field_rank = field.rank().to_index();
    let field_file = field.file().to_index();

    let mut direction: i8 = match color {
        Color::WHITE => 1,
        Color::BLACK => -1,
    };

    // promotion_type is only set if pawn promotes
    let promotion_type: Option<PieceType> =
        if field.rank() == Rank::SEVEN && color == Color::WHITE
                || field.rank() == Rank::TWO && color == Color::BLACK {
            Some(PieceType::QUEEN) // TODO: are all possible promotions added or just Queen?
        } else {
            None
        };

    // the field straight ahead has to be unoccupied to move
    let forward: Field = Field::new(File::from_index(field_file), Rank::from_index(field_rank + direction as usize));
    if board.get_piece(&forward).is_none() {
        moves.push(ChessMove::new(field.clone(), forward, promotion_type));
    }

    // the fields diagonal have to be occupied by opponent
    // the diagonal field does not exist at the edge of the board
    let diagonal_left: Option<Field> = if field_file > 0 {
        Some(Field::new(File::from_index(field_file - 1), Rank::from_index(field_rank + direction as usize)))
    } else {
        None
    };
    let diagonal_right: Option<Field> = if field_file < 7 {
        Some(Field::new(File::from_index(field_file + 1), Rank::from_index(field_rank + direction as usize)))
    } else {
        None
    };
    if diagonal_left.is_some_and(|d| board.get_piece(&d).is_some_and(|p|p.color() != color)) {
        moves.push(ChessMove::new(field.clone(), diagonal_left.unwrap(), promotion_type));
    }
    if diagonal_right.is_some_and(|d| board.get_piece(&d).is_some_and(|p|p.color() != color)) {
        moves.push(ChessMove::new(field.clone(), diagonal_right.unwrap(), promotion_type));
    }

    // the two fields ahead of the start line have to be unoccupied
    if field.rank() == Rank::TWO && color == Color::WHITE
            || field.rank() == Rank::SEVEN && color == Color::BLACK {
        let double_forward = Field::new(
            File::from_index(field_file),
            Rank::from_index(field_rank + 2*direction as usize)
        );
        if board.get_piece(&forward).is_none() && board.get_piece(&double_forward).is_none() {
            moves.push(ChessMove::new(field.clone(), double_forward, promotion_type));
        }
    }

    if board.en_passant_field().is_some() {
        if color == Color::WHITE && field.rank() == Rank::FIVE
                && board.en_passant_field().unwrap().rank() == Rank::SIX
                && (field.file().to_index() as isize
                    - board.en_passant_field().unwrap().file().to_index() as isize).abs() == 1 {
            moves.push(ChessMove::new(
                field.clone(),
                board.en_passant_field().unwrap(),
                promotion_type)
            );
        } else if color == Color::BLACK && field.rank() == Rank::FOUR
            && board.en_passant_field().unwrap().rank() == Rank::THREE
            && (field.file().to_index() as isize
            - board.en_passant_field().unwrap().file().to_index() as isize).abs() == 1 {
            moves.push(ChessMove::new(
                field.clone(),
                board.en_passant_field().unwrap(),
                promotion_type)
            );
        }
    }

    moves
}