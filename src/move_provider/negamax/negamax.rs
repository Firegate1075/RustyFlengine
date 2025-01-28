use log::{debug, info};
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::MoveProvider;
use crate::rules::piece_rule::PieceRule;
use crate::rules::RulesProvider;

#[derive(Debug)]
pub struct Negamax;

use rayon::prelude::*;
use crate::converter::converter::Converter;
use crate::move_provider::negamax::recursive_negamax_task::{recursive_negamax_task, Evaluation, NegamaxTaskContext};

impl MoveProvider for Negamax {
    fn get_recommended_moves(&mut self, board: &Board, options: Options) -> Vec<ChessMove> {
        info!("Starting calculation of negamax for position {:?}.", Converter::convert_board_to_string(board));
        let moves = PieceRule::get_legal_moves(board, &board.next_color());

        info!("Got legal moves: {:?}", &moves);

        let mut evaluated_moves: Vec<(ChessMove, Evaluation)> =
            moves.par_iter().map( |mov|
                (mov.clone(), recursive_negamax_task(
            NegamaxTaskContext {
                        board: board.clone(),
                        chess_move: *mov,
                        current_level: 1,
                        player_color: board.next_color(),
                        max_level: options.recursion_depth() as i32,
                    }
                ))
            ).collect();

        debug!("Calculated negamax moves");

        // sort the moves based on eval
        evaluated_moves.sort_by_key(|(_, eval)| eval.clone());
        debug!("Evaluation by negamax: {:?}", evaluated_moves);
        // discard the eval and put the moves into vector
        evaluated_moves.into_iter().map(|(mov, _)| mov).rev().collect()

    }
}