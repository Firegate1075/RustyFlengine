use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::rank::Rank;
use crate::datamodel::enums::file::File;
use crate::datamodel::field::Field;

mod datamodel;
mod move_provider;
mod converter;
mod rules;
mod controller;

fn main() {
    println!("Hello, world!");

    let mut board: Board = Board::new();
    let move_to_play: ChessMove = ChessMove::new(Field::new(File::E, Rank::TWO), Field::new(File::E, Rank::FOUR), None);
    board.play_move(&move_to_play);
}
