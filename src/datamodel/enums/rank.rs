use log::error;
use strum_macros::EnumIter;

#[derive(Debug, Copy, PartialEq, EnumIter)]
#[repr(usize)]
#[derive(Clone)]
pub enum Rank {
    ONE     = 0,
    TWO     = 1,
    THREE   = 2,
    FOUR    = 3,
    FIVE    = 4,
    SIX     = 5,
    SEVEN   = 6,
    EIGHT   = 7,
}

impl Rank {
    pub fn to_index(&self) -> usize {
        match self {
            Rank::ONE   => 0,
            Rank::TWO   => 1,
            Rank::THREE => 2,
            Rank::FOUR  => 3,
            Rank::FIVE  => 4,
            Rank::SIX   => 5,
            Rank::SEVEN => 6,
            Rank::EIGHT => 7,
        }
    }

    pub fn from_index(i: usize) -> Rank {
        match i {
            0 => Rank::ONE,
            1 => Rank::TWO,
            2 => Rank::THREE,
            3 => Rank::FOUR,
            4 => Rank::FIVE,
            5 => Rank::SIX,
            6 => Rank::SEVEN,
            7 => Rank::EIGHT,
            _ => panic!("Invalid rank number {}", i),
        }
    }

    pub fn from_string(s: &str) -> Rank {
        match s {
            "1" => Rank::ONE,
            "2" => Rank::TWO,
            "3" => Rank::THREE,
            "4" => Rank::FOUR,
            "5" => Rank::FIVE,
            "6" => Rank::SIX,
            "7" => Rank::SEVEN,
            "8" => Rank::EIGHT,
            _ => panic!("Invalid rank number {}", s),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Rank::ONE => "1",
            Rank::TWO => "2",
            Rank::THREE => "3",
            Rank::FOUR => "4",
            Rank::FIVE => "5",
            Rank::SIX => "6",
            Rank::SEVEN => "7",
            Rank::EIGHT => "8",
        }
    }
}