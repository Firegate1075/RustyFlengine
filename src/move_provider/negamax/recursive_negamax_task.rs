use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::color::Color;


use crate::datamodel::field::Field;
use crate::rules::piece_rule::PieceRule;
use crate::rules::RulesProvider;

use rayon::prelude::*;

pub struct NegamaxTaskContext {
    pub board: Board,
    pub chess_move: ChessMove,
    pub current_level: i32,
    pub player_color: Color,
    pub max_level: i32,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Evaluation {
    Rating(i32),
    OpponentHasCheckmateIn(i32),
    PlayerHasCheckmateIn(i32),
    Draw,
}

impl Evaluation {
    pub fn negate(&self) -> Evaluation {
        match self {
            Evaluation::Rating(rating) => Evaluation::Rating(-*rating),
            Evaluation::OpponentHasCheckmateIn(n) => Evaluation::PlayerHasCheckmateIn(*n),
            Evaluation::PlayerHasCheckmateIn(n) => Evaluation::OpponentHasCheckmateIn(*n),
            Evaluation::Draw => Evaluation::Draw,
        }
    }

    // our naive guess has to be updated with the opponents best response
    // our move is only good, if opponent does not have a good response
    pub fn update_with_opponents_best_response(self, opponent_move: &Self) -> Self {
        // if we only have a estimated rating, update it with the evaluation of the best response
        if let Evaluation::Rating(rating) = self {
            match opponent_move {
                // e.g. if the opponent has a move that gains a rook (+5 for them)
                // after we take a bishop (+3 for us), that gives a total eval of -2 for us
                Evaluation::Rating(opponent_rating) => {
                    Evaluation::Rating(rating-opponent_rating)
                },

                // if opponents opponent has mate in n moves after the move
                // that means we have mate in n+1 moves
                // e.g. after our candidate move, our opponent evaluates the position as
                // "my opponent (that is us) has mate in one, no matter what i do"
                // then that means if we play the move in question, we have forced mate in two
                Evaluation::OpponentHasCheckmateIn(n) => {
                    Evaluation::PlayerHasCheckmateIn(*n+1)
                }
                // if opponent has mate in n after the move
                // opponent has mate in n after the move
                Evaluation::PlayerHasCheckmateIn(n) => {
                    Evaluation::OpponentHasCheckmateIn(*n)
                }
                // if our opponent is sure its a forced draw, then it is a forced draw
                Evaluation::Draw => {
                    Evaluation::Draw
                }
            }
        } else {
            // if we have a more concrete evaluation of the position
            // (forced mate, forced draw) just keep that.
            self
        }
    }
}

// support comparison and max value computation

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Evaluation) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Evaluation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Evaluation::OpponentHasCheckmateIn(n1) => {
                match other {
                    Evaluation::OpponentHasCheckmateIn(n2) => {
                        // longer opponent mate is better
                        // e.g. mate in _1_ is worse than mate in _2_
                        n1.cmp(n2)
                    }
                    // mate is worse than anything else
                    _ => {std::cmp::Ordering::Less}
                }
            }
            Evaluation::PlayerHasCheckmateIn(n1) => {
                match other {
                    Evaluation::PlayerHasCheckmateIn(n2) => {
                        // shorter opponent mate is better
                        // e.g. mate in _1_ is better than mate in _2_
                        n2.cmp(n1)
                    }
                    // mate is better than anything else
                    _ => {std::cmp::Ordering::Greater}
                }
            }
            Evaluation::Draw => {
                match other {
                    Evaluation::Draw => {std::cmp::Ordering::Equal},
                    // draw is worse than win
                    Evaluation::PlayerHasCheckmateIn(_) => {std::cmp::Ordering::Less},
                    // draw is better than loss
                    Evaluation::OpponentHasCheckmateIn(_) => {std::cmp::Ordering::Greater},
                    Evaluation::Rating(rating) => {
                        // draw is better than losing position
                        0.cmp(rating)
                    }
                }
            }
            Evaluation::Rating(rating) => {
                match other {
                    // better eval is better
                    Evaluation::Rating(rating2) => {
                        rating.cmp(rating2)
                    }
                    // forced mate is better
                    Evaluation::PlayerHasCheckmateIn(_) => {std::cmp::Ordering::Less},
                    // eval is better than forced mate
                    Evaluation::OpponentHasCheckmateIn(_) => {std::cmp::Ordering::Greater},
                    Evaluation::Draw => {
                        // winning position is better than draw
                        rating.cmp(&0)
                    }
                }
            }
        }
    }
}

pub fn recursive_negamax_task(context: NegamaxTaskContext) -> Evaluation {
    let mut new_board = context.board.clone();
    let opponent_color: Color = match context.player_color {
        Color::BLACK => Color::WHITE,
        Color::WHITE => Color::BLACK,
    };
    
    
    // rate the move for the given position by itself (without calculating more than the one move
    // into the future) and play the move
    let evaluation: Evaluation = rate_move(&mut new_board, &context.chess_move, context.player_color);

    // if we reached max recursion level or if we have a concrete eval, return the evaluation
    if context.current_level >= context.max_level
        || !matches!(evaluation, Evaluation::Rating(_))
    {
        return evaluation;
    }

    // if we only have a estimated rating of the move, recursively check the possible responses
    let legal_moves = PieceRule::get_legal_moves(&new_board, &new_board.next_color());

    // for each response move compute the rating
    legal_moves.par_iter().map( |legal_move|
         recursive_negamax_task(NegamaxTaskContext {
            board: new_board.clone(),
            chess_move: *legal_move,
            current_level: context.current_level + 1,
            player_color: opponent_color,
             ..context
        })
    // update our evaluation with the best response
    // if there is no legal move for the opponent, its stalemate or checkmate
    // since checkmate is handled above, it is fine to return draw here for now as default
    ).max().map_or(Evaluation::Draw, |best_response|
        evaluation.update_with_opponents_best_response(&best_response))
}

/// rates the given move for the player of the given color
fn rate_move(board: &mut Board, chess_move: &ChessMove, player_color: Color) -> Evaluation {
    let mut rating: i32 = 0;
    let to_field: Field = *chess_move.to_field();

    let opponent_color: Color = match player_color {
        Color::BLACK => Color::WHITE,
        Color::WHITE => Color::BLACK,
    };

    // get the piece that will get taken
    let piece_to_hit = board.get_piece(&to_field).clone();

    // play the move
    board.play_move(chess_move);
    
    // if opponent is mated, player has mate in one
    if PieceRule::is_checkmated(board, &opponent_color) {
        return Evaluation::PlayerHasCheckmateIn(1)
    }
    
    // TODO: DRAW logic here
    
    if let Some(piece_to_hit) = piece_to_hit {
        // own piece being taken is bad
        if piece_to_hit.color() == player_color {
            rating -= piece_to_hit.piece_type().value() as i32;
        } else {
            rating += piece_to_hit.piece_type().value() as i32;
        }
    }

    if PieceRule::is_checked(board, &player_color) {
        rating -= 1;
    }
    if PieceRule::is_checked(board, &opponent_color) {
        rating += 1;
    }
    
    // negamax works by returning the negative of the rating of the move
    Evaluation::Rating(rating)
}