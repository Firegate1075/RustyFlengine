use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
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

    pub fn convert_board_to_string(board: &Board) -> String {
        let mut fen: String = "".to_string();
        let mut empty_field_counter: i32 = 0;
        let mut is_castling_possible: bool = false;

        // get pieces
        for rank in (0..8).rev() {
            for file in 0..8 {
                let current_piece: &Option<Piece> = board.get_piece(
                    &Field::new(
                        File::from_index(file as usize),
                        Rank::from_index(rank as usize))
                );

                // check for piece
                if current_piece.is_some() {
                    // field has piece
                    let mut current_piece_fen_representation: String = match current_piece
                        .unwrap()
                        .piece_type() {
                            PieceType::ROOK => "r".to_string(),
                            PieceType::KNIGHT => "n".to_string(),
                            PieceType::BISHOP => "b".to_string(),
                            PieceType::QUEEN => "q".to_string(),
                            PieceType::KING => "k".to_string(),
                            PieceType::PAWN => "p".to_string(),
                    };

                    // if piece color is white, change piece fen string to uppercase
                    if current_piece.unwrap().color() == Color::WHITE {
                        current_piece_fen_representation = current_piece_fen_representation
                            .to_ascii_uppercase();
                    }

                    // add empty fields to fen string
                    if empty_field_counter > 0 {
                        fen += empty_field_counter.to_string().as_str();
                        empty_field_counter = 0;
                    }

                    // add piece to fen string
                    fen += current_piece_fen_representation.as_str();
                } else {
                    // field is empty
                    empty_field_counter += 1;
                }

            }

            // after rank is done
            // add empty fields to fen string
            if empty_field_counter > 0 {
                fen += empty_field_counter.to_string().as_str();
                empty_field_counter = 0;
            }

            // check whether it is not the last line and only then add a slash
            if rank != 0 {
                fen += "/";
            }
        }
        // pieces done

        // get next move color
        if board.next_color() == Color::WHITE {
            fen += " w";
        } else {
            fen += " b";
        }

        // get castling information
        fen += " ";

        if board.white_can_castle_long() {
            fen += "Q";
            is_castling_possible = true;
        }
        if board.white_can_castle_short() {
            fen += "K";
            is_castling_possible = true;
        }
        if board.black_can_castle_long() {
            fen += "q";
            is_castling_possible = true;
        }
        if board.black_can_castle_short() {
            fen += "k";
            is_castling_possible = true;
        }

        // check whether no castling is possible
        if !is_castling_possible {
            fen += "-";
        }

        // get en passant information
        if board.en_passant_field().is_some() {
            fen += " ";
            fen += board.en_passant_field().unwrap().file().to_str().to_ascii_lowercase().as_str();
            fen += board.en_passant_field().unwrap().rank().to_str();
        } else {
            fen += " -";
        }

        // get half moves information
        // not implemented yet, hardcoded dummy value
        fen += " 0";

        // get number of move
        // not implemented yet, hardcoded dummy value
        fen += " ";
        fen += board.move_counter().to_string().as_str();

        fen
    }

    /// Converts the given string to a move object
    pub fn convert_string_to_move(input: &String) -> ChessMove {
        let input: String = input.clone().to_ascii_lowercase();

        let chess_move: ChessMove;

        if input.len() == 4 {
            chess_move = ChessMove::new(
                Field::new(
                    File::from_string(input.chars().nth(0).unwrap().to_string().as_str()),
                    Rank::from_string(input.chars().nth(1).unwrap().to_string().as_str()))
                , Field::new(
                    File::from_string(input.chars().nth(2).unwrap().to_string().as_str()),
                    Rank::from_string(input.chars().nth(3).unwrap().to_string().as_str())
                ),
                None
            );
        } else {
            // promotion has length 5
            let promotion_type: Option<PieceType> = match input.chars().nth(4).unwrap() {
                'q' => Some(PieceType::QUEEN),
                'b' => Some(PieceType::BISHOP),
                'n' => Some(PieceType::KNIGHT),
                'r' => Some(PieceType::ROOK),
                _   => None,
            };

            chess_move = ChessMove::new(
                Field::new(
                    File::from_string(input.chars().nth(0).unwrap().to_string().as_str()),
                    Rank::from_string(input.chars().nth(1).unwrap().to_string().as_str()))
                , Field::new(
                    File::from_string(input.chars().nth(2).unwrap().to_string().as_str()),
                    Rank::from_string(input.chars().nth(3).unwrap().to_string().as_str())
                ),
                Some(promotion_type.expect("invalid promotion type"))
            );
        }

        chess_move
    }

    /// Converts a move object to a string
    pub fn convert_move_to_string(input: &ChessMove) -> String {
        let mut chess_move: String = "".to_string();
        let from_field = input.from_field();
        let to_field = input.to_field();

        chess_move += from_field.file().to_str().to_ascii_lowercase().as_str();
        chess_move += from_field.rank().to_str();
        chess_move += to_field.file().to_str().to_ascii_lowercase().as_str();
        chess_move += to_field.rank().to_str();

        if input.promote_to().is_some() {
            chess_move += match input.promote_to().unwrap() {
                PieceType::ROOK => "r",
                PieceType::KNIGHT => "n",
                PieceType::BISHOP => "b",
                PieceType::QUEEN => "q",
                _ => "",
            };
        }

        chess_move
    }

    pub fn sanitize_move(board: &Board, chess_move: &ChessMove) -> ChessMove {
        let mut sanitized_move: ChessMove = chess_move.clone();

        // if king is moved along the first or eight rank
        if board.get_piece(chess_move.from_field())
            .is_some_and(|piece| piece.piece_type() == PieceType::KING)
            && chess_move.from_field().file() == File::E
            && (
                chess_move.from_field().rank() == Rank::ONE
                && chess_move.from_field().rank() == Rank::ONE
                || chess_move.to_field().rank() == Rank::EIGHT
                && chess_move.to_field().rank() == Rank::EIGHT
            )
        {
            if chess_move.to_field().file() == File::H {
                // represent short castles as moving from e to g file
                sanitized_move.set_to_field(Field::new(File::G, chess_move.from_field().rank()));
            } else if chess_move.to_field().file() == File::A {
                // represent long castles as moving from e to c file
                sanitized_move.set_to_field(Field::new(File::C, chess_move.from_field().rank()));
            }
        }

        sanitized_move
    }
}