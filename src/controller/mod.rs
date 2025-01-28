use std::sync::{LazyLock, Mutex};
use log::{info, warn};
use rand::{random_range};
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::endgame::Endgame;
use crate::move_provider::minmax::min_max::MinMax;
use crate::move_provider::MoveProvider;
use crate::move_provider::negamax::negamax::Negamax;
use crate::move_provider::openings::Openings;

static MOVE_PROVIDERS: LazyLock<Mutex<[Box<dyn MoveProvider + Sync + Send>; 4]>> =
    LazyLock::new(|| Mutex::new([
        Box::new(Openings::new()),
        Box::new(Endgame),
        Box::new(Negamax),
        Box::new(MinMax),
]));

pub struct Controller;



impl Controller {
    pub fn give_move(board: &Board, options: &Options) -> Option<ChessMove> {

        info!("Move requested");

        for move_provider in MOVE_PROVIDERS.lock().unwrap().iter_mut() {
            let move_provider = move_provider.as_mut();
            info!("Requesting moves from: [{move_provider:?}]");

            // get the moves from the move provider in descending order of rating (1st = best)
            let moves: Vec<ChessMove> = move_provider.get_recommended_moves(board, options.clone());

            if !moves.is_empty() {
                let best_move = moves.get(
                    // raising a value between 0 and 1 to a high power returns a value closer to 0
                    // thus, a higher difficulty will yield values closer to the beginning, which are
                    // the better moves.
                    // the value is capped at 1 with modulo operator (for negative difficulty)
                    (random_range(0.0..1.0f64).powi(options.difficulty().value()) % 1.0f64 * moves.len() as f64).floor() as usize
                );

                info!("Best move is [{best_move:?}] by [{move_provider:?}]");
                return Some(best_move.unwrap().clone());
            }
        }

    warn!("No possible moves were found.");
    None
    }
}