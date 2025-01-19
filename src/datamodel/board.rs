use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::color::Color;
use crate::datamodel::enums::rank::Rank;
use crate::datamodel::enums::piece_type::PieceType;
use crate::datamodel::enums::file::File;
use crate::datamodel::piece::Piece;
use super::field::Field;

pub struct Board {
    /// Represents the en passant field.
    /// If not existing the value is None.
    en_passant_field: Option<Field>,

    /// Represents the chess board with all figures on it.
    /// First index is line, second index is row
    pieces: [[Option<Piece>; 8]; 8],

    /// Stores the color of the player to move
    next_color: Color,

    /// Number of the next move to be done on the board. This is important for fen string support.
    move_counter: u16,

    /// Indicates whether white has short castling rights
    white_can_castle_short: bool,
    /// Indicates whether white has long castling rights
    white_can_castle_long: bool,
    /// Indicates whether black has short castling rights
    black_can_castle_short: bool,
    /// Indicates whether black has long castling rights
    black_can_castle_long: bool,
}


impl Board {
    pub fn new() -> Board {
        Board {
            en_passant_field: None,
            pieces: [const {[const { None }; 8]}; 8],
            next_color: Color::WHITE,
            move_counter: 1,
            white_can_castle_short: true,
            white_can_castle_long: true,
            black_can_castle_short: true,
            black_can_castle_long: true,
        }
    }

    pub fn get_piece(&self, field: &Field) -> &Option<Piece> {
        &self.pieces[field.rank().to_index()][field.file().to_index()]
    }

    pub fn set_piece(&mut self, piece: Option<Piece>, field: &Field) {
        self.pieces[field.rank().to_index()][field.file().to_index()] = piece;
    }

    pub fn play_move(&mut self, chess_move: &ChessMove) -> () {
        let piece_from = self.get_piece(chess_move.from_field()).unwrap_or_else(|| panic!("Cannot play move. There is no piece on field {:?}", chess_move.from_field()));

        // check whether move affects future castling rights and set flags accordingly
        if piece_from.piece_type() == PieceType::KING {
            // moving the king loses castling rights
            if piece_from.color() == Color::WHITE {
                self.white_can_castle_long = false;
                self.white_can_castle_short = false;
            } else {
                self.black_can_castle_long = false;
                self.black_can_castle_short = false;
            }
        } else if piece_from.piece_type() == PieceType::ROOK {
            // moving the rook loses castling rights
            if piece_from.color() == Color::WHITE {
                if chess_move.from_field().file() == File::A {
                    self.white_can_castle_long = false;
                } else if chess_move.from_field().file() == File::H {
                    self.white_can_castle_short = false;
                }
            } else {
                if chess_move.from_field().file() == File::A {
                    self.black_can_castle_long = false;
                } else if chess_move.from_field().file() == File::H {
                    self.black_can_castle_short = false;
                }
            }
        }

        // check if the move to play is an en passant move
        if piece_from.piece_type() == PieceType::PAWN && (chess_move.from_field().rank() != chess_move.to_field().rank() || chess_move.from_field().file() != chess_move.to_field().file()) {
            // remove the captured pawn. It is on the same line as the starting field of the move and the same row as the ending field of the move
            self.set_piece(None, &Field::new(chess_move.to_field().file(), chess_move.from_field().rank()))
        }

        // set the piece on the to-field
        self.set_piece(*self.get_piece(chess_move.from_field()), chess_move.to_field());

        //delete piece from from-field
        self.set_piece(None, chess_move.from_field());

        // check for castling
        if self.get_piece(chess_move.to_field()).unwrap().piece_type() == PieceType::KING {
            // check for white short castling
            if chess_move.from_field().rank() == Rank::ONE && chess_move.from_field().file() == File::E
                && chess_move.to_field().rank() == Rank::ONE && chess_move.to_field().file() == File::G {
                // move rook
                self.set_piece(Some(Piece::new(Color::WHITE, PieceType::ROOK)), &Field::new(File::F, Rank::ONE));
                self.set_piece(None, &Field::new(File::H, Rank::ONE));
            }
            else if chess_move.from_field().rank() == Rank::ONE && chess_move.from_field().file() == File::E
                && chess_move.to_field().rank() == Rank::ONE && chess_move.to_field().file() == File::C {
                // move rook
                self.set_piece(Some(Piece::new(Color::WHITE, PieceType::ROOK)), &Field::new(File::D, Rank::ONE));
                self.set_piece(None, &Field::new(File::A, Rank::ONE));
            }
            else if chess_move.from_field().rank() == Rank::EIGHT && chess_move.from_field().file() == File::E
                && chess_move.from_field().rank() == Rank::EIGHT && chess_move.from_field().file() == File::G {
                // move rook
                self.set_piece(Some(Piece::new(Color::BLACK, PieceType::ROOK)), &Field::new(File::F, Rank::EIGHT));
                self.set_piece(None, &Field::new(File::H, Rank::EIGHT));
            }
            else if chess_move.from_field().rank() == Rank::EIGHT && chess_move.from_field().file() == File::E
                && chess_move.from_field().rank() == Rank::EIGHT && chess_move.from_field().file() == File::C {
                // move rook
                self.set_piece(Some(Piece::new(Color::BLACK, PieceType::ROOK)), &Field::new(File::D, Rank::EIGHT));
                self.set_piece(None, &Field::new(File::A, Rank::EIGHT));
            }
        }

        // check whether move is a promotion
        if chess_move.promote_to().is_some() {
            self.set_piece(Some(Piece::new(piece_from.color(), chess_move.promote_to().unwrap())), &chess_move.to_field());
        }

        // check for en passant and set en_passant_field
        if self.get_piece(chess_move.to_field()).unwrap().piece_type() == PieceType::PAWN {
            if chess_move.from_field().rank() == Rank::TWO {
                if chess_move.to_field().rank() == Rank::FOUR {
                    self.en_passant_field = Some(Field::new(chess_move.from_field().file(), Rank::THREE));
                } else {
                    self.en_passant_field = None;
                }
            } else if chess_move.from_field().rank() == Rank::SEVEN {
                if chess_move.to_field().rank() == Rank::FIVE {
                    self.en_passant_field = Some(Field::new(chess_move.from_field().file(), Rank::SIX));
                } else {
                    self.en_passant_field = None;
                }
            } else {
                self.en_passant_field = None;
            }
        } else {
            self.en_passant_field = None;
        }

        self.next_color = match self.next_color {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };

        self.move_counter += 1;
    }

    pub fn piece_count(&self) -> u16 {
        self.pieces.iter().map(
            |array| array.iter()
                .filter(|elem| elem.is_some())
                .count()
        ).sum::<usize>() as u16
    }

    pub fn black_can_castle_long(&self) -> bool {
        self.black_can_castle_long
    }

    pub fn black_can_castle_short(&self) -> bool {
        self.black_can_castle_short
    }

    pub fn white_can_castle_long(&self) -> bool {
        self.white_can_castle_long
    }

    pub fn white_can_castle_short(&self) -> bool {
        self.white_can_castle_short
    }

    pub fn move_counter(&self) -> u16 {
        self.move_counter
    }

    pub fn next_color(&self) -> Color {
        self.next_color
    }

    pub fn en_passant_field(&self) -> &Option<Field> {
        &self.en_passant_field
    }

    pub fn set_en_passant_field(&mut self, en_passant_field: Option<Field>) {
        self.en_passant_field = en_passant_field;
    }

    pub fn set_next_color(&mut self, next_color: Color) {
        self.next_color = next_color;
    }

    pub fn set_move_counter(&mut self, move_counter: u16) {
        self.move_counter = move_counter;
    }

    pub fn set_white_can_castle_short(&mut self, white_can_castle_short: bool) {
        self.white_can_castle_short = white_can_castle_short;
    }

    pub fn set_white_can_castle_long(&mut self, white_can_castle_long: bool) {
        self.white_can_castle_long = white_can_castle_long;
    }

    pub fn set_black_can_castle_short(&mut self, black_can_castle_short: bool) {
        self.black_can_castle_short = black_can_castle_short;
    }

    pub fn set_black_can_castle_long(&mut self, black_can_castle_long: bool) {
        self.black_can_castle_long = black_can_castle_long;
    }
}
