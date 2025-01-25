use std::error::Error;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Clone, EnumIter)]
pub enum Difficulty {
    EASY,
    NORMAL,
    HARD,
}

#[derive(Clone, Debug)]
pub struct ConversionError;

impl Difficulty {
    pub fn value(&self) -> i32 {
        match *self {
            Difficulty::EASY => 1,
            Difficulty::NORMAL => 8,
            Difficulty::HARD => 100,
        }
    }

    pub fn readable_string(&self) -> String{
        match *self {
            Difficulty::EASY => "Easy".to_string(),
            Difficulty::NORMAL => "Normal".to_string(),
            Difficulty::HARD => "Hard".to_string(),
        }
    }
    pub fn from_str(string: &str) -> Result<Difficulty, ConversionError> {
        match string {
            "EASY" => Ok(Difficulty::EASY),
            "NORMAL" => Ok(Difficulty::NORMAL),
            "HARD" => Ok(Difficulty::HARD),
            _ => Err(ConversionError),
        }
    }
}