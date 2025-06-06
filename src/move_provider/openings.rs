use crate::converter::converter::Converter;
use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::options::Options;
use crate::move_provider::MoveProvider;

use reqwest::blocking::get;
use log::{ info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
struct OpeningMove {
    pub uci: String,
    #[serde(rename = "averageRating")]
    pub average_rating: i32,
    pub white: i32,
    pub draws: i32,
    pub black: i32,
}

#[derive(Deserialize)]
struct OpeningResponse {
    pub white: i32,
    pub draws: i32,
    pub black: i32,
    pub moves: Vec<OpeningMove>,
}

/// An implementation of MoveProvider that uses the Lichess Opening Explorer
/// to find the best move in an opening situation.
#[derive(Debug)]
pub struct Openings {
    out_of_opening: bool,
}

impl Openings {
    pub fn new() -> Openings {
        Openings { out_of_opening: false }
    }
}

impl MoveProvider for Openings{
    fn get_recommended_moves(&mut self, board: &Board, options: Options) -> Vec<ChessMove> {
        let fen_string: String = Converter::convert_board_to_string(&board);

        if self.out_of_opening {
            info!("We are out of the opening");
            return Vec::new();
        }

        info!("Requesting opening moves for: [{}]", fen_string);

        let body = get("https://explorer.lichess.ovh/masters?fen=".to_string() + fen_string.as_str() + "&topGames=0");
        match body {
            Ok(body) => {
                let response_string: String = match body.text() {
                    Ok(text) => text,
                    Err(e) => { warn!("Could not convert response to string: {}", e); return Vec::new(); }
                };

                info!("Received answer [{}]", response_string);

                let opening_data: OpeningResponse = match serde_json::from_str(response_string.as_str()) {
                    Ok(data) => data,
                    Err(e) => { warn!("Could not parse response string: {}", e); return Vec::new(); }
                };

                info!("Received: [{} moves]", opening_data.moves.len());

                if opening_data.moves.is_empty() {
                    self.out_of_opening = true;
                    info!("There are no more opening moves available");
                }

                opening_data.moves.into_iter()
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