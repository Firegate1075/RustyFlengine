use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;

/// Provides functionality to obtain recommended moves for a given chess situation.
pub trait MoveProvider {
    /// Provides a vector of recommended ChessMoves for the given position on the Board.
    fn get_recommended_moves(board: &Board, options: Options) -> Vec<ChessMove>;
}