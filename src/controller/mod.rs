use std::sync::{LazyLock, Mutex};
use log::{info, warn};
use rand::{random_range};
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::endgame::Endgame;
use crate::move_provider::minmax::min_max::MinMax;
use crate::move_provider::MoveProvider;
use crate::move_provider::openings::Openings;

static MOVE_PROVIDERS: LazyLock<Mutex<[Box<dyn MoveProvider + Sync + Send>; 3]>> = LazyLock::new(|| Mutex::new([
    Box::new(Openings::new()),
    Box::new(Endgame),
    Box::new(MinMax),
]));

pub struct Controller;



impl Controller {
    pub fn give_move(board: &Board, options: &Options) -> Option<ChessMove> {

        info!("Move requested");

        for mut move_provider in MOVE_PROVIDERS.lock().unwrap().iter_mut() {
            let move_provider = move_provider.as_mut();
            info!("Requesting moves from: [{move_provider:?}]");

            let moves: Vec<ChessMove> = move_provider.get_recommended_moves(board, options.clone());

            if !moves.is_empty() {
                let best_move = moves.get(
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