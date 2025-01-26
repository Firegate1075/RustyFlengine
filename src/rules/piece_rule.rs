use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::color::Color;
use crate::datamodel::enums::file::File;
use crate::datamodel::enums::piece_type::PieceType;
use crate::datamodel::enums::rank::Rank;
use crate::datamodel::field::Field;

use itertools::Itertools;
use log::{debug, error, info};
use strum::IntoEnumIterator;
use crate::converter::converter::Converter;
use crate::rules::RulesProvider;

/// Implementation of the individual rules for each chess piece.
/// Also contains the methods isFieldCovered and isLegalMove.
pub struct PieceRule;

impl PieceRule {
    /// Returns all possible moves of a piece on the given field.
    /// May include illegal moves (e.g. due to pins, check, etc.)
    pub fn get_moves_of_piece(board: &Board, field: &Field) -> Vec<ChessMove> {
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

    /// Returns whether the king of the given color is in check
    pub fn is_checked(board: &Board, color: &Color) -> bool {
        debug!("Checking if {:?} is in check", color);

        let opponent_color: Color = match color {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };

        for rank in Rank::iter() {
            for file in File::iter() {
                let field = Field::new(file, rank);
                if board.get_piece(&field).is_some_and(|piece|
                    piece.piece_type() == PieceType::KING && piece.color() == *color
                ) {
                    return is_field_covered(board, &field, opponent_color);
                }
            }
        }

        error!("King of color {} could not be found", color);
        error!("Position is {}", Converter::convert_board_to_string(board));
        false
    }

    /// Returns whether the king of the given color is checkmated
    pub fn is_checkmated(board: &Board, color: &Color) -> bool {
        // its checkmate, when the player is in check and has no legal moves
        Self::is_checked(board, color)
            && Self::get_legal_moves(board, color).is_empty()
            && board.next_color() == *color
    }
}

impl RulesProvider for PieceRule {
    fn get_legal_moves(board: &Board, color: &Color) -> Vec<ChessMove> {
        debug!("Calculating legal moves for {}", color);
        let mut moves: Vec<ChessMove> = Vec::new();

        for rank in Rank::iter() {
            for file in File::iter() {
                let field = Field::new(file, rank);

                if board.get_piece(&field).is_some_and(|piece|
                    piece.color() == *color
                ) {
                    debug!("piece on {:?} has moves {:?}", field,  Self::get_moves_of_piece(board, &field));
                    moves.append(&mut Self::get_moves_of_piece(board, &field));
                }
            }
        }

        debug!("Found possible moves {:?}", moves);

        moves.iter().filter(|possible_move| {
            let mut cloned_board = board.clone();
            cloned_board.play_move(*possible_move);
            // own king must not be in check after move
            let would_be_check = Self::is_checked(&cloned_board, color);
            if  would_be_check {
                debug!("Excluding move {:?} because it would put own king in check", possible_move);
            }
            !would_be_check
        }).cloned().collect()
    }
}



const ROOK_DIRECTIONS: [[isize; 2]; 4] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
];

const BISHOP_DIRECTIONS: [[isize; 2]; 4] = [
    [1, 1],
    [-1, 1],
    [1, -1],
    [-1, -1],
];

const QUEEN_DIRECTIONS: [[isize; 2]; 8] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
    [1, 1],
    [-1, 1],
    [1, -1],
    [-1, -1],
];

const KNIGHT_DIRECTIONS: [[isize; 2]; 8] = [
    [-2, -1],
    [-2,  1],
    [ 2, -1],
    [ 2,  1],
    [-1, -2],
    [-1,  2],
    [ 1, -2],
    [ 1,  2],
];

/// Returns a list of all fields visible in a list of directions.
/// That is, every field attacked or defended by the bishop/rook/queen on square `field`.
///
/// This function returns all the fields visible in a given direction, so it is only useful for
/// bishops, rooks and queens. Pieces with a limited vision, like pawns, kings and knights must be
/// handled separately.
fn get_visible_fields_along_direction(board: &Board, field: &Field, directions: Vec<[isize; 2]>) -> Vec<Field> {
    let mut visible_fields: Vec<Field> = Vec::new();
    let field_rank = field.rank().to_index() as isize;
    let field_file = field.file().to_index() as isize;

    for direction in directions.iter() {
        let mut rank = field_rank;
        let mut file = field_file;
        while rank + direction[0] >= 0 && rank + direction[0] < 8
            && file + direction[1] >= 0 && file + direction[1] < 8 {

            let next_field = Field::new(
                File::from_index((file + direction[1]) as usize),
                Rank::from_index((rank + direction[0]) as usize),
            );

            visible_fields.push(next_field);

            if board.get_piece(&next_field).is_none() {
                // next field in direction is unoccupied => continue
                rank += direction[0];
                file += direction[1];
            } else {
                // found piece in sight line => stop
                break;
            }
        }
    }
    visible_fields
}

fn get_legal_moves_king(board: &Board, field: &Field) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let field_rank = field.rank().to_index() as isize;
    let field_file = field.file().to_index() as isize;
    let color: Color = board.get_piece(field).unwrap().color();
    let opponent_color: Color = match color {
        Color::BLACK => Color::WHITE,
        Color::WHITE => Color::BLACK,
    };

    // TODO: Handling of kingField. maybe make functions member functions of struct and add field
    // maybe replace

    // add all possible castling moves
    moves.append(&mut get_castling_moves(board,field).clone());


    for (rank, file) in (-1..=1).cartesian_product(-1..=1) {
        if (rank, file) == (0, 0) {
            continue;
        }

        if field_rank + rank >= 0 && field_rank + rank < 8
            && field_file + file >= 0 && field_file + file < 8
        {
            let adjacent_field = Field::new(
                File::from_index((field_file + file) as usize),
                Rank::from_index((field_rank + rank) as usize),
            );

            if board.get_piece(&adjacent_field).is_none_or(|piece|
                piece.color() != color)
                // field must not be covered by opponent
                && !is_field_covered(board, &adjacent_field, opponent_color)
            {
                moves.push(ChessMove::new(
                    field.clone(),
                    adjacent_field,
                    None
                ));
            }
        }
    }

    moves
}

fn get_legal_moves_queen(board: &Board, field: &Field) -> Vec<ChessMove> {
    let color: Color = board.get_piece(field).unwrap().color();
    let visible_fields = get_visible_fields_along_direction(board, field, QUEEN_DIRECTIONS.to_vec());

    // you can only move to empty fields or capture opponents pieces
    visible_fields.iter().filter( |possible_field: &&Field|
        board.get_piece(*possible_field).is_none_or(|piece| piece.color() != color)
    ).map(|to_field|
        ChessMove::new(
            field.clone(),
            to_field.clone(),
            None,
        )
    ).collect()
}

fn get_legal_moves_bishop(board: &Board, field: &Field) -> Vec<ChessMove> {
    let color: Color = board.get_piece(field).unwrap().color();
    let visible_fields = get_visible_fields_along_direction(board, field, BISHOP_DIRECTIONS.to_vec());

    // you can only move to empty fields or capture opponents pieces
    visible_fields.iter().filter( |possible_field: &&Field|
        board.get_piece(possible_field).is_none_or(|piece| piece.color() != color)
    ).map(|to_field|
        ChessMove::new(
            field.clone(),
            to_field.clone(),
            None,
        )
    ).collect()
}

fn get_legal_moves_knight(board: &Board, field: &Field) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let color: Color = board.get_piece(field).unwrap().color();
    let field_rank = field.rank().to_index() as isize;
    let field_file = field.file().to_index() as isize;


    for direction in KNIGHT_DIRECTIONS.iter() {
        if field_rank + direction[0] >= 0 && field_rank + direction[0] < 8
                && field_file + direction[1] >= 0 && field_file + direction[1] < 8 {
            let next_field = Field::new(
                File::from_index((field_file + direction[1]) as usize),
                Rank::from_index((field_rank + direction[0]) as usize),
            );

            // the knight can move to empty squares or capture opponents pieces
            if board.get_piece(&next_field).is_none_or(|piece| piece.color() != color) {
                moves.push(ChessMove::new(
                   field.clone(),
                   next_field,
                   None,
                ));
            }
        }
    }

    moves
}

fn get_legal_moves_rook(board: &Board, field: &Field) -> Vec<ChessMove> {
    let color: Color = board.get_piece(field).unwrap().color();
    let visible_fields = get_visible_fields_along_direction(board, field, ROOK_DIRECTIONS.to_vec());

    // you can only move to empty fields or capture opponents pieces
    visible_fields.iter().filter( |possible_field: &&Field|
        board.get_piece(possible_field).is_none_or(|piece| piece.color() != color)
    ).map(|to_field|
        ChessMove::new(
            field.clone(),
            to_field.clone(),
            None,
        )
    ).collect()
}

/// Returns all possible moves of a pawn at given position.
fn get_legal_moves_pawn(board: &Board, field: &Field) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let color: Color = board.get_piece(field).unwrap().color();
    let field_rank = field.rank().to_index();
    let field_file = field.file().to_index();

    let direction: i8 = match color {
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
    let forward: Field = Field::new(File::from_index(field_file), Rank::from_index((field_rank as i8 + direction) as usize));
    if board.get_piece(&forward).is_none() {
        moves.push(ChessMove::new(field.clone(), forward, promotion_type));
    }

    // the fields diagonal have to be occupied by opponent
    // the diagonal field does not exist at the edge of the board
    let diagonal_left: Option<Field> = if field_file > 0 {
        Some(Field::new(File::from_index(field_file - 1), Rank::from_index((field_rank as i8 + direction) as usize)))
    } else {
        None
    };
    let diagonal_right: Option<Field> = if field_file < 7 {
        Some(Field::new(File::from_index(field_file + 1), Rank::from_index((field_rank as i8 + direction ) as usize)))
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

/// Returns the possible castle moves of a king.
///
/// A castle move is only possible if:
///     - Neither king nor rook have moved
///     - There are no pieces between king and rook
///     - The king is not currently in check
///     - The king does not pass through or ends up at a covered square
/// see [Castling - Requirements](https://en.wikipedia.org/wiki/Castling#Requirements)
fn get_castling_moves(board: &Board, field: &Field) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let color: Color = board.get_piece(field).unwrap().color();
    let opponent_color: Color = match color {
        Color::BLACK => Color::WHITE,
        Color::WHITE => Color::BLACK,
    };
    let has_kingside_castling_rights = match color {
        Color::BLACK => board.black_can_castle_short(),
        Color::WHITE => board.white_can_castle_short(),
    };
    let has_queenside_castling_rights = match color {
        Color::BLACK => board.black_can_castle_long(),
        Color::WHITE => board.white_can_castle_long(),
    };

    // NOTE: many of the checks of the original Flengine are redundant, because they
    // are contained within castling rights, e.g. that rook and king are at the correct positions

    // king is not in check
    if !is_field_covered(board, field, opponent_color) {

        let field_b: Field = Field::new(File::B, field.rank());
        let field_c: Field = Field::new(File::C, field.rank());
        let field_d: Field = Field::new(File::D, field.rank());

        let field_f: Field = Field::new(File::F, field.rank());
        let field_g: Field = Field::new(File::G, field.rank());

        // queen side castling
        if has_queenside_castling_rights
            // there are no pieces between king and rook
            && board.get_piece(&field_b).is_none()
            && board.get_piece(&field_c).is_none()
            && board.get_piece(&field_d).is_none()
            // the king does not move through or into check
            && !is_field_covered(board, &field_c, opponent_color)
            && !is_field_covered(board, &field_d, opponent_color)
        {
            moves.push(ChessMove::new(
                field.clone(),
                field_c.clone(),
                None,
            ))
        }

        if has_kingside_castling_rights
            // there are no pieces between king and rook
            && board.get_piece(&field_f).is_none()
            && board.get_piece(&field_g).is_none()
            // the king does not move through or into check
            && !is_field_covered(board, &field_f, opponent_color)
            && !is_field_covered(board, &field_g, opponent_color)
        {
            moves.push(ChessMove::new(
                field.clone(),
                field_g.clone(),
                None,
            ))
        }
    }

    moves
}



/// Returns if a king of given color is in a one-field-radius of the given field.
fn is_king_in_range(board: &Board, field: &Field, color: Color) -> bool {
    let field_rank = field.rank().to_index() as isize;
    let field_file = field.file().to_index() as isize;

    for (rank, file) in (-1..=1).cartesian_product(-1..=1) {
        if (rank, file) == (0, 0) {
            continue;
        }

        if field_rank + rank >= 0 && field_rank + rank < 8
            && field_file + file >= 0 && field_file + file < 8
        {
            let adjacent_field = Field::new(
                File::from_index((field_file + file) as usize),
                Rank::from_index((field_rank + rank) as usize),
            );

            if board.get_piece(&adjacent_field).is_some_and(|piece|
                piece.color() == color && piece.piece_type() == PieceType::KING)
            {
                return true;
            }
        }
    }

    false
}

/// Returns if the given field is covered by a piece of the given color.
fn is_field_covered(board: &Board, field: &Field, color: Color) -> bool {
    debug!("Checking if {:?} is covered by {:?}", field, color);
    let pawn_direction: isize = match color {
        Color::WHITE => -1,
        Color::BLACK => 1,
    };

    let field_rank = field.rank().to_index() as isize;
    let field_file = field.file().to_index() as isize;



    // field is covered by pawn
    if color == Color::WHITE && field_rank > 0 || color == Color::BLACK && field_rank < 7 {
        let diagonal_left: Option<Field> = if field_file > 0 {
            Some(Field::new(
                File::from_index((field_file - 1) as usize),
                Rank::from_index((field_rank + pawn_direction) as usize)
            ))
        } else {
            None
        };
        let diagonal_right: Option<Field> = if field_file < 7 {
            Some(Field::new(
                File::from_index((field_file + 1) as usize),
                Rank::from_index((field_rank + pawn_direction) as usize)
            ))
        } else {
            None
        };

        if diagonal_left.is_some_and(|d| board.get_piece(&d).is_some_and(|p|
                p.color() == color && p.piece_type() == PieceType::PAWN))
            || diagonal_right.is_some_and(|d| board.get_piece(&d).is_some_and(|p|
                p.color() == color && p.piece_type() == PieceType::PAWN))
        {
            debug!("Field is covered by a pawn");
            return true;
        }
    }

    // field is covered by knight
    for direction in KNIGHT_DIRECTIONS.iter() {
        if field_rank + direction[0] >= 0 && field_rank + direction[0] < 8
            && field_file + direction[1] >= 0 && field_file + direction[1] < 8 {
            let next_field = Field::new(
                File::from_index((field_file + direction[1]) as usize),
                Rank::from_index((field_rank + direction[0]) as usize),
            );

            if board.get_piece(&next_field).is_some_and(|piece|
                piece.color() == color && piece.piece_type() == PieceType::KNIGHT)
            {
                debug!("Field is covered by a knight");
                return true;
            }
        }
    }

    // field is covered by rook or queen
    if get_visible_fields_along_direction(board, field, ROOK_DIRECTIONS.to_vec()).iter()
        .filter_map(|f|
            *board.get_piece(f)
        ).any(|p|
            p.color() == color &&
                (p.piece_type() == PieceType::QUEEN || p.piece_type() == PieceType::ROOK)
        )
    {
        debug!("Field is covered by a rook or a queen");
        return true;
    }

    // field is covered by bishop or queen
    if get_visible_fields_along_direction(board, field, BISHOP_DIRECTIONS.to_vec()).iter()
        .filter_map(|f|
            *board.get_piece(f)
        ).any(|p|
        p.color() == color &&
            (p.piece_type() == PieceType::QUEEN || p.piece_type() == PieceType::BISHOP)
    )
    {
        debug!("Field is covered by a bishop or a queen");
        return true;
    }

    // field is covered by king
    for (rank, file) in (-1..=1).cartesian_product(-1..=1) {
        if (rank, file) == (0, 0) {
            continue;
        }

        if field_rank + rank >= 0 && field_rank + rank < 8
            && field_file + file >= 0 && field_file + file < 8
        {
            let adjacent_field = Field::new(
                File::from_index((field_file + file) as usize),
                Rank::from_index((field_rank + rank) as usize),
            );

            if board.get_piece(&adjacent_field).is_some_and(|piece|
                piece.color() == color && piece.piece_type() == PieceType::KING)
            {
                debug!("Field is covered by a king");
                return true;
            }
        }
    }

    debug!("Field is not covered.");
    false
}