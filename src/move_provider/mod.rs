use std::fmt::Debug;
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;

pub mod openings;
pub mod endgame;

pub mod minmax;
/// Provides functionality to obtain recommended moves for a given chess situation.
pub trait MoveProvider: Debug {
    /// Provides a vector of recommended ChessMoves for the given position on the Board.
    fn get_recommended_moves(&mut self, board: &Board, options: Options) -> Vec<ChessMove>;
}