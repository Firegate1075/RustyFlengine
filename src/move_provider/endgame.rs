use crate::converter::converter::Converter;
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::move_provider::MoveProvider;

use reqwest::blocking::get;
use log::{ info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
struct EndgameMove {
    pub uci: String,
    pub zeroing: bool,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub category: EndgameCategory,
}

#[derive(Deserialize)]
enum EndgameCategory {
    WIN,
    LOSS,
}

#[derive(Deserialize)]
struct EndgameResponse {
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub category: EndgameCategory,
    pub moves: Vec<EndgameMove>,
}

/// An implementation of MoveProvider that uses the Lichess Opening Explorer
/// to find the best move in an opening situation.
pub struct Endgame;

impl MoveProvider for Endgame{
    fn get_recommended_moves(board: &Board, options: Options) -> Vec<ChessMove> {
        let piece_count = board.piece_count();
        if piece_count > 7 {
            info!("The given board has more than 7 pieces left: [{}]", piece_count);
            return Vec::new();
        }

        let fen_string: String = Converter::convert_board_to_string(&board);

        info!("Requesting endgame moves for: [{}]", fen_string);

        let body = get("https://tablebase.lichess.ovh/standard?fen=".to_string() + fen_string.as_str());
        match body {
            Ok(body) => {
                let response_string: String = match body.text() {
                    Ok(text) => text,
                    Err(e) => { warn!("Could not convert response to string: {}", e); return Vec::new(); }
                };

                let endgame_data: EndgameResponse = match serde_json::from_str(response_string.as_str()) {
                    Ok(data) => data,
                    Err(e) => { warn!("Could not parse response string: {}", e); return Vec::new(); }
                };

                info!("Received: [{} moves]", endgame_data.moves.len());

                endgame_data.moves.into_iter()
                    .map(|mov| Converter::convert_string_to_move(&mov.uci))
                    .map(|mov| Converter::sanitize_move(&board, &mov))
                    .collect()
            },
            Err(error) => {
                warn!("Request failed: {}", error);

                Vec::new()
            },
        }
    }
}