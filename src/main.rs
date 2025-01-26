use crate::datamodel::board::Board;
use crate::datamodel::chess_move::ChessMove;
use crate::datamodel::enums::rank::Rank;
use crate::datamodel::enums::file::File;
use crate::datamodel::field::Field;

use log4rs;
use log_panics;
use log::LevelFilter::*;
use crate::converter::converter::Converter;
use crate::converter::request_handler::RequestHandler;
use crate::logging::LoggingConfig;

mod datamodel;
mod move_provider;
mod converter;
mod rules;
mod controller;
mod logging;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("Hello, world!");

    let logging_config = LoggingConfig::new(
        false,
        true,
        true,
        "%Y%m%d-%H%M%S-%3f".to_string(),
        "./flengine-".to_string(),
        Info
    );

    let handle = logging::setup_logger(&logging_config);

    log_panics::init();

    RequestHandler::start_up(logging_config, handle);
}
