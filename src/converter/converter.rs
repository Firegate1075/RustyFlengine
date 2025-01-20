use crate::datamodel::board::Board;
use crate::datamodel::enums::color::Color;
use crate::datamodel::enums::file::File;
use crate::datamodel::enums::piece_type::PieceType;
use crate::datamodel::enums::rank::Rank;
use crate::datamodel::field::Field;
use crate::datamodel::piece::Piece;

pub struct Converter;

/// Converts a given string to a board object
impl Converter {
    pub fn convert_string_to_board(input: String) -> Board {
        let mut board: Board = Board::new();

        // startposition
        if input == "startpos" {
            // set up white pieces
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::ROOK)),    &Field::new(File::A, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::KNIGHT)),  &Field::new(File::B, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::BISHOP)),  &Field::new(File::C, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::QUEEN)),   &Field::new(File::D, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::KING)),    &Field::new(File::E, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::BISHOP)),  &Field::new(File::F, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::KNIGHT)),  &Field::new(File::G, Rank::ONE));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::ROOK)),    &Field::new(File::H, Rank::ONE));

            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::A, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::B, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::C, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::D, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::E, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::F, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::G, Rank::TWO));
            board.set_piece(Some(Piece::new(Color::WHITE, PieceType::PAWN)),    &Field::new(File::H, Rank::TWO));

            // set up black pieces
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::ROOK)),    &Field::new(File::A, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::KNIGHT)),  &Field::new(File::B, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::BISHOP)),  &Field::new(File::C, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::QUEEN)),   &Field::new(File::D, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::KING)),    &Field::new(File::E, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::BISHOP)),  &Field::new(File::F, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::KNIGHT)),  &Field::new(File::G, Rank::EIGHT));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::ROOK)),    &Field::new(File::H, Rank::EIGHT));

            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::A, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::B, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::C, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::D, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::E, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::F, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::G, Rank::SEVEN));
            board.set_piece(Some(Piece::new(Color::BLACK, PieceType::PAWN)),    &Field::new(File::H, Rank::SEVEN));

        } else {
            // fen string
            let split: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
            let pos: &String = &split[0];

            // set pieces

            // split the rows
            let mut rank_count: i32 = 7;
            let pos_splitted = pos.split("/");
            for row in pos_splitted {
                let mut file_count: i32 = 0;
                for c in row.chars() {
                    let mut piece: Option<Piece> = None;

                    if c.is_ascii_digit() {
                        // add empty fields on rowCount
                        file_count = file_count + c.to_digit(10).unwrap() as i32;
                    } else {
                        // field is not empty
                        if c.is_ascii_uppercase() {
                            //white
                            piece = match c.to_ascii_lowercase() {
                                'r' => Some(Piece::new(Color::WHITE, PieceType::ROOK)),
                                'n' => Some(Piece::new(Color::WHITE, PieceType::KNIGHT)),
                                'b' => Some(Piece::new(Color::WHITE, PieceType::BISHOP)),
                                'q' => Some(Piece::new(Color::WHITE, PieceType::QUEEN)),
                                'k' => Some(Piece::new(Color::WHITE, PieceType::KING)),
                                'p' => Some(Piece::new(Color::WHITE, PieceType::PAWN)),
                                _ => None
                            };
                        } else {
                            //black
                            piece = match c.to_ascii_lowercase() {
                                'r' => Some(Piece::new(Color::BLACK, PieceType::ROOK)),
                                'n' => Some(Piece::new(Color::BLACK, PieceType::KNIGHT)),
                                'b' => Some(Piece::new(Color::BLACK, PieceType::BISHOP)),
                                'q' => Some(Piece::new(Color::BLACK, PieceType::QUEEN)),
                                'k' => Some(Piece::new(Color::BLACK, PieceType::KING)),
                                'p' => Some(Piece::new(Color::BLACK, PieceType::PAWN)),
                                _ => None
                            };
                        }
                        board.set_piece(piece, &Field::new(File::from_index(file_count as usize), Rank::from_index(rank_count as usize)));
                        file_count += 1;
                    }
                }
                rank_count -= 1;
            }

            // decode next move color
            if split[1] == "w" {
                board.set_next_color(Color::WHITE);
            } else {
                board.set_next_color(Color::BLACK);
            }

            // decode castling rights
            let castling_string: &String = &split[2];

            board.set_white_can_castle_short(false);
            board.set_white_can_castle_long(false);
            board.set_black_can_castle_long(false);
            board.set_black_can_castle_short(false);

            for c in castling_string.chars() {
                match c {
                    'K' => board.set_white_can_castle_short(true),
                    'Q' => board.set_white_can_castle_long(true),
                    'k' => board.set_black_can_castle_short(true),
                    'q' => board.set_black_can_castle_long(true),
                    _ => (),
                }
            }

            // decode possible en passant
            let en_passant_string: &String = &split[3];
            if en_passant_string != "-" {
                let en_passant_field = Field::new(
                    File::from_string(en_passant_string.chars().nth(0).unwrap().to_string().as_str()),
                    Rank::from_index(en_passant_string.chars().nth(1).unwrap().to_digit(10).unwrap() as usize)
                );
                board.set_en_passant_field(Some(en_passant_field));
            }

            // decode half moves

            // decode move number
            board.set_move_counter(split[5].parse::<u16>().unwrap())

        }


        board
    }
}