use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::color::Color;


use crate::datamodel::field::Field;
use crate::rules::piece_rule::PieceRule;
use crate::rules::RulesProvider;

use rayon::prelude::*;

pub struct MinmaxTaskContext {
    pub board: Board,
    pub chess_move: ChessMove,
    pub current_level: i32,
    pub player_color: Color,
    pub max_level: i32,
}

pub fn recursive_minmax_task(context: MinmaxTaskContext) -> i32 {
    let mut new_board = context.board.clone();

    let rating: i32 = rate_move(&new_board, &context.chess_move, context.player_color);
    new_board.play_move(&context.chess_move);

    if context.current_level >= context.max_level || rating.abs() > 60 {
        return rating;
    }

    // TODO: Flengine says board.next_color() but should it not be new_board
    let legal_moves = PieceRule::get_legal_moves(&new_board, &context.board.next_color());


    // for each legal move compute the rating
    legal_moves.par_iter().map( |legal_move|
         recursive_minmax_task(MinmaxTaskContext{
            board: new_board.clone(),
            chess_move: *legal_move,
            current_level: context.current_level + 1,
            ..context
        })
    ).max().map_or(-100, |rating_increment| rating_increment + rating)
}

/// rates the given move for the player of the given color
fn rate_move(board: &Board, chess_move: &ChessMove, player_color: Color) -> i32 {
    let mut evaluation: i32 = 0;
    let field: Field = *chess_move.to_field();

    let opponent_color: Color = !player_color;

    let piece_to_hit = board.get_piece(&field);

    if let Some(piece_to_hit) = piece_to_hit {
        if board.next_color() == player_color {
            evaluation -= piece_to_hit.piece_type().value() as i32;
        } else {
            evaluation += piece_to_hit.piece_type().value() as i32;
        }
    }

    if PieceRule::is_checked(board, &player_color) {
        evaluation += 1;
    }
    if PieceRule::is_checked(board, &opponent_color) {
        evaluation -= 1;
    }
    if PieceRule::is_checkmated(board, &player_color) {
        evaluation += 100;
    }
    if PieceRule::is_checkmated(board, &opponent_color) {
        evaluation -= 100;
    }

    evaluation
}