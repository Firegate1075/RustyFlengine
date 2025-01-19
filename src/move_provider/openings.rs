use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::move_provider::MoveProvider;

struct OpeningMove {
    pub uci: String,
    pub average_rating: i32,
    pub white: i32,
    pub draws: i32,
    pub black: i32,
}


struct OpeningResponse {
    pub white: i32,
    pub draws: i32,
    pub black: i32,
    pub moves: Vec<OpeningMove>,
}

/// An implementation of MoveProvider that uses the Lichess Opening Explorer
/// to find the best move in an opening situation.
pub struct Openings{

}
impl MoveProvider for Openings{
    fn get_recommended_moves(board: Board, options: Options) -> Vec<ChessMove> {
        todo!()
    }
}