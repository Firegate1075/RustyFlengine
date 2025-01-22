use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::color::Color;

pub mod piece_rule;


/// provides functionality to compute the legal moves in a given position
pub trait RulesProvider {
    /// returns a Vector of legal moves of a piece color in a given position
    fn get_legal_moves(board: &Board, color: &Color) -> Vec<ChessMove>;
}