use log::{debug, info};
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::MoveProvider;
use crate::rules::piece_rule::PieceRule;
use crate::rules::RulesProvider;

#[derive(Debug)]
pub struct MinMax;

use rayon::prelude::*;
use crate::converter::converter::Converter;
use crate::move_provider::minmax::recursive_minmax_task::{recursive_minmax_task, MinmaxTaskContext};

impl MoveProvider for MinMax {
    fn get_recommended_moves(&mut self, board: &Board, options: Options) -> Vec<ChessMove> {
        info!("Starting calculation of minmax for position {:?}.", Converter::convert_board_to_string(board));
        let mut moves = PieceRule::get_legal_moves(board, &board.next_color());

        info!("Got legal moves: {:?}", &moves);

        let mut evaluated_moves: Vec<(ChessMove, i32)> =
            moves.par_iter().map( |mov|
                (mov.clone(), recursive_minmax_task(
            MinmaxTaskContext {
                        board: board.clone(),
                        chess_move: *mov,
                        current_level: 1,
                        player_color: board.next_color(),
                        MAXLEVEL: options.recursion_depth() as i32,
                    }
                ))
            ).collect();

        debug!("Calculated minmax moves");

        evaluated_moves.sort_by_key(|(_, rating)| *rating);
        evaluated_moves.into_iter().map(|(mov, _)| mov).collect()

    }
}