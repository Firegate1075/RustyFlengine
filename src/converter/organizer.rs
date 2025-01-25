use std::future::{Future, IntoFuture};
use crate::converter::converter::Converter;
use crate::datamodel::board::Board;
use crate::datamodel::options::Options;

use tokio_util::sync::CancellationToken;
use tokio::{runtime, select, task};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use crate::controller::Controller;
use crate::datamodel::chess_move::ChessMove;


pub struct CancellationError;

pub struct Organizer {
    options: Options,
    board: Board,
    board_string: String,
    move_strings: Vec<String>,
    cancellation_token: Option<CancellationToken>
}

impl Organizer {
    pub fn new(options: Options, fen_board: String, moves: Vec<String>) -> Organizer {
        Organizer {
            options,
            board: Converter::convert_string_to_board(fen_board.clone()),
            board_string: fen_board,
            move_strings: moves,
            cancellation_token: None,
        }
    }

    /// Calculate the next best move to be executed on the actual board.
    /// Returns a future that contains the move as uci compatible move string once the calculation is done.
    pub fn calculate_next_move_async(&mut self) -> JoinHandle<Result<String, CancellationError>> {

        let cancellation_token = CancellationToken::new();

        self.cancellation_token = Some(cancellation_token.clone());

        let moves = self.move_strings.clone();
        let board = self.board.clone();
        let options = self.options.clone();

        tokio::spawn( async move {
            select! {
                _ = cancellation_token.cancelled() => Err(CancellationError),
                result = Self::calculate_next_move(moves, board, options) => {
                    Ok(result)
                }
            }
        })
    }

    /// Calculate the next best move to be executed on the actual board.
    /// Returns the best move as uci compatible move string.
    async fn calculate_next_move(move_strings: Vec<String>, mut board: Board, options: Options) -> String {
        move_strings.iter().for_each(|m|
            board.play_move(&Converter::convert_string_to_move(m))
        );
        let best_move: ChessMove = Controller::give_move(&board, &options).unwrap();
        Converter::convert_move_to_string(&best_move)
    }

    pub fn stop_calculations(&self) {
        self.cancellation_token.as_ref().unwrap().cancel();
    }
}