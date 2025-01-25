use log::{info, warn};
use rand::{random_range};
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::endgame::Endgame;
use crate::move_provider::minmax::min_max::MinMax;
use crate::move_provider::MoveProvider;
use crate::move_provider::openings::Openings;

pub struct Controller;



impl Controller {
    pub fn give_move(board: &Board, options: &Options) -> Option<ChessMove> {

        let move_provider_hierarchy: [Box<dyn MoveProvider + Sync>; 3] = [
            Box::new(Openings),
            Box::new(Endgame),
            Box::new(MinMax),
        ];

        for move_provider in move_provider_hierarchy.iter() {
            let move_provider = move_provider.as_ref();
            info!("Requesting moves from: [{move_provider:?}]");

            let moves: Vec<ChessMove> = move_provider.get_recommended_moves(board, options.clone());

            let best_move = moves.get(
                random_range(0.0..1.0f64).powi(options.difficulty().value() % 1 * moves.len() as i32).floor() as usize
            );

            info!("Best move is [{best_move:?}] by [{move_provider:?}]");
            return Some(best_move.unwrap().clone());
        }

    warn!("No possible moves were found.");
    None
    }
}