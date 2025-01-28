use std::future::Future;
use std::io::{stdin, stdout, Error, Write};
use futures::{FutureExt, TryFutureExt};
use log::{info, warn};
use log4rs::Handle;
use crate::datamodel::enums::difficulty::Difficulty;
use crate::datamodel::options::Options;

use strum::IntoEnumIterator;
use crate::converter::organizer::Organizer;
use crate::logging;
use crate::logging::LoggingConfig;

pub struct RequestHandler;

const ENGINE_NAME: &str = "Flengine";
const ENGINE_AUTHOR: &str = "TeamFlyndre";

impl RequestHandler {
    pub fn start_up(mut logging_config: LoggingConfig, logging_handle: Handle) -> () {
        let mut options = Options::from_default();

        let mut position: String = String::new();
        let mut moves: Vec<String> = Vec::new();

        let mut input = String::new();
        let mut is_running: bool = true;

        let mut organizer: Option<Organizer> = None;

        info!("Engine initialized and ready.");

        while is_running {
            input.clear();
            let _ = stdin().read_line(&mut input);
            info!("Received input string: [{}]", input);

            let mut splitted_input: Vec<&str> = input.split_whitespace().collect();

            loop {
                match splitted_input[0].to_lowercase().as_str() {
                    "uci" => {
                        println!("id name {}", ENGINE_NAME);
                        println!("id author {}", ENGINE_AUTHOR);
                        info!("Sent name and author to gui.");
                        // options
                        // difficulty value
                        let mut difficulty_options: String =
                            "option name Difficulty type combo default "
                        .to_string() + Difficulty::NORMAL.readable_string().as_str();
                        for difficulty in Difficulty::iter() {
                            difficulty_options = difficulty_options + " var " + difficulty.readable_string().as_str();
                        }
                        println!("{}", difficulty_options);
                        info!("Indicated difficulty option to gui: {}", difficulty_options);
                        // recursion depth
                        let recursive_depth_options: String =
                            String::from("option name RecursiveDepth type spin default ")
                            + options.recursion_depth().to_string().as_str()
                            + " min 1 max 10";
                        println!("{}", recursive_depth_options);
                        // uciok
                        println!("uciok");
                        info!("Finished initial communication with gui.");
                    }
                    "setoption" => {
                        // read the given option and change the value in the options object accordingly
                        info!("Recognized setoption command from gui.");
                        if splitted_input.len() > 3 {
                            match splitted_input[2] {
                                "Difficulty" => {
                                    let difficulty = Difficulty::from_str(&splitted_input[4]);
                                    match difficulty {
                                        Ok(difficulty) => {
                                            info!("Changed option difficulty to [{}]", difficulty.readable_string());
                                            options.set_difficulty(difficulty);
                                        }
                                        Err(_) => {
                                            warn!("The value [{}] is not a valid difficulty.", splitted_input[4]);
                                        }
                                    }
                                }
                                "RecursiveDepth" => {
                                    if let Ok(recursive_depth) = splitted_input[4].parse::<u32>() {
                                        info!("Changed option recursiveDepth to [{}]", recursive_depth);
                                        options.set_recursion_depth(recursive_depth);
                                    } else {
                                        warn!("The value [{}] is not a valid number.", splitted_input[4]);
                                    }
                                }
                                _ => {
                                    warn!("The value [{}] is not a supported option", splitted_input[2]);
                                }
                            }
                        }
                    }
                    "isready" => {
                        // no initialization needed here at the moment, so indicated engine is ready
                        println!("readyok");
                    }
                    "ucinewgame" => {
                        // ignore
                        info!("Recognized ucinewgame.");
                    }
                    "position" => {
                        // get position
                        info!("Recognized position command.");
                        if splitted_input.len() > 1 {
                            position = splitted_input[1].to_string();

                            match position.as_str() {
                                "fen" => {
                                    // the fen string is also split, so compute it from its parts
                                    // the fen string starts at element 3 and is made of 6 parts
                                    position = splitted_input.iter().skip(2).take(6)
                                        .map(|s| s.to_string())
                                        // concatenate the strings with spaces
                                        .reduce(|fen, elem| fen + " " + &elem)
                                        .unwrap();
                                    
                                    // get moves
                                    // position fen <fen1> <fen2> <fen3> <fen4> <fen5> <fen6> moves <move1> <move2> ... 
                                    if splitted_input.len() > 9 {
                                        if splitted_input[8] == "moves" {
                                            // take all the move strings after "moves", which is the 4th element
                                            moves = splitted_input.iter().skip(9).map(|s| s.to_string()).collect();
                                        }
                                    }
                                }
                                "startpos" => {
                                    // leave position string as startpos
                                    // get moves
                                    if splitted_input.len() > 3 {
                                        if splitted_input[2] == "moves" {
                                            moves = splitted_input.iter().skip(3).map(|s| s.to_string()).collect();
                                        }
                                    }
                                }
                                _ => { panic!("The value [{}] is not a valid position.", position); }
                            };
                            
                            // computing is started with the go command
                        }
                    }
                    "go" => {
                        info!("Recognized go command. Starting calculation...");
                        organizer = Some(Organizer::new(
                            options.clone(),
                            position.clone(),
                            moves.clone(),
                        ));
                        let future_move = organizer.as_mut().unwrap().calculate_next_move_async();
                        let future_move = future_move.then(|mut s| async move {
                            let result = s.unwrap();
                            info!("Calculation finished. Best move: {}", result.as_ref().ok().unwrap());
                            println!("bestmove {}", result.as_ref().ok().unwrap());
                        });
                        tokio::spawn(future_move);
                    }
                    "stop" => {
                        // indicate gui asked to send the move
                        // not used yet
                        info!("Recognized stop command.");
                    }
                    "debug" => {
                        info!("Recognized debug command.");
                        if splitted_input.len() > 1 {
                            match splitted_input[1] {
                                "on" => {
                                    // enable uci logging
                                    info!("Set debug mode to [on]");
                                    logging_config.set_uci_enabled(false);
                                    logging::update_logger(&logging_config, &logging_handle);
                                }
                                "off" => {
                                    // disable uci logging
                                    info!("Set debug mode to [off]");
                                    logging_config.set_uci_enabled(true);
                                    logging::update_logger(&logging_config, &logging_handle);
                                }
                                _ => {
                                    warn!("The value [{}] is not a valid value for debug.", splitted_input[1]);
                                }
                            }
                        }
                    }
                    "quit" => {
                        info!("Recognized quit command. Shutting down engine.");
                        if organizer.as_ref().is_some() {
                            organizer.as_ref().unwrap().stop_calculations();
                        }
                        is_running = false;
                    }
                    _ => {
                        if splitted_input.len() > 1 {
                            info!("Unrecognized command {}, trying to parse next input.", splitted_input[0]);
                            splitted_input = splitted_input.iter().skip(1).cloned().collect();
                        }
                        continue;
                    }
                };
                // leave the inner loop if input is recognized
                break;
            }
        }

    }
}